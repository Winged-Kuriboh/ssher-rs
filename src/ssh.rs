use crate::model::Server;
use base64::Engine;
use base64::engine::general_purpose;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size};
use russh::keys::*;
use russh::*;
use std::env;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

pub(crate) async fn exec(server: Server) -> anyhow::Result<()> {
    let mut ssh = Session::connect(server).await?;

    ssh.shell().await?;
    ssh.close().await?;

    Ok(())
}

struct Client {}

impl Client {
    fn new() -> Self {
        Self {}
    }
}

impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> anyhow::Result<bool, Self::Error> {
        Ok(true)
    }
}

struct RawModeGuard {
    enabled: bool,
}

impl RawModeGuard {
    fn new() -> anyhow::Result<Self> {
        let mut enabled = is_raw_mode_enabled()?;
        if !enabled {
            enable_raw_mode()?;
            enabled = true;
        }
        Ok(Self { enabled })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if self.enabled {
            disable_raw_mode().unwrap();
        }
    }
}

/// This struct is a convenience wrapper
/// around a russh client
/// that handles the input/output event loop
pub struct Session {
    server_host: String,
    session: client::Handle<Client>,
}

impl Session {
    async fn connect(server: Server) -> anyhow::Result<Self> {
        let config = client::Config::default();
        let config = Arc::new(config);

        // try resolving the host as an IP address
        let host = if server.host.parse::<std::net::IpAddr>().is_ok() {
            server.host.clone()
        } else {
            let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
            let response = resolver
                .lookup_ip(server.host.as_str())
                .await
                .map_err(|e| anyhow::anyhow!("DNS lookup failed: {}", e))?;
            response
                .iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No IP addresses returned for {}", server.host))?
                .to_string()
        };

        let mut session = client::connect(config, (host, server.port), Client::new()).await?;

        let auth_rs = match server.password {
            Some(ref password) if !password.is_empty() => {
                let password = general_purpose::STANDARD.decode(password)?;
                session
                    .authenticate_password(server.user, String::from_utf8(password)?)
                    .await?
            }
            _ => {
                let identity_file = match server.identity_file {
                    Some(ref identity_file) => identity_file,
                    None => "~/.ssh/id_rsa",
                };

                let expanded_path = shellexpand::tilde(identity_file).into_owned();
                let key_pair = load_secret_key(expanded_path, None)?;
                session
                    .authenticate_publickey(
                        server.user,
                        PrivateKeyWithHashAlg::new(
                            Arc::new(key_pair),
                            session.best_supported_rsa_hash().await?.flatten(),
                        ),
                    )
                    .await?
            }
        };

        if !auth_rs.success() {
            anyhow::bail!("😿 Authentication failed.")
        }

        Ok(Self {
            server_host: server.host.clone(),
            session,
        })
    }

    async fn shell(&mut self) -> anyhow::Result<()> {
        // We're using `crossterm` to put the terminal into raw mode, so that we can
        // display the output of interactive applications correctly
        let _raw_mode = RawModeGuard::new()?;

        let mut channel = self.session.channel_open_session().await?;

        let (mut col, mut row) = size()?;
        // Request an interactive PTY from the server
        channel
            .request_pty(
                true,
                &env::var("TERM").unwrap_or("xterm".into()),
                col as u32,
                row as u32,
                0,
                0,
                &[], // ideally you want to pass the actual terminal modes here
            )
            .await?;
        channel.request_shell(true).await?;

        let mut stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut buf = vec![0; 1024];

        #[cfg(unix)]
        // Spawn a task to handle the SIGTERM signal
        tokio::spawn(Self::handle_terminate_signal(self.server_host.clone()));

        loop {
            let (new_col, new_row) = size()?;
            if (col, row) != (new_col, new_row) {
                col = new_col;
                row = new_row;
                channel
                    .window_change(new_col as u32, new_row as u32, 0, 0)
                    .await?;
            }

            // Handle one of the possible events:
            tokio::select! {
                // There's terminal input available from the user
                r = stdin.read(&mut buf)=> {
                    match r {
                        Ok(0) => {
                            channel.eof().await?;
                            break;
                        }
                        // Send it to the server
                        Ok(n) => channel.data(&buf[..n]).await?,
                        Err(e) => return Err(e.into()),
                    };
                },
                // There's an event available on the session channel
                Some(msg) = channel.wait() => {
                    match msg {
                        // Write data to the terminal
                        ChannelMsg::Data { ref data } => {
                            stdout.write_all(data).await?;
                            stdout.flush().await?;
                        }
                        // The server has closed the channel
                        ChannelMsg::ExitStatus { .. } =>{
                            Self::close_connection(self.server_host.clone(), &mut stdout).await?;
                            // stdout.write_all(format!("Connection to {} closed.\r\n", self.server_host.clone()).as_bytes()).await?;
                            // stdout.flush().await?;
                            channel.eof().await?;
                            break;
                        }
                        _ => {}
                    }
                },
            }
        }

        Ok(())
    }

    async fn close(&mut self) -> anyhow::Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }

    #[cfg(unix)]
    async fn handle_terminate_signal(server_host: String) {
        if let Ok(mut signal) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            signal.recv().await;

            if let Ok(true) = is_raw_mode_enabled() {
                let _ = disable_raw_mode();
            }
            let mut stdout = tokio::io::stdout();
            Self::close_connection(server_host, &mut stdout)
                .await
                .unwrap();

            std::process::exit(0);
        }
    }

    async fn close_connection(
        server_host: String,
        stdout: &mut tokio::io::Stdout,
    ) -> anyhow::Result<()> {
        stdout
            .write_all(format!("Connection to {} closed.\r\n", server_host).as_bytes())
            .await?;
        stdout.flush().await?;

        Ok(())
    }
}
