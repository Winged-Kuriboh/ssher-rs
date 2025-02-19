use crate::model::Server;
use base64::engine::general_purpose;
use base64::Engine;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size};
use russh::keys::*;
use russh::*;
use std::env;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::AsyncResolver;

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

/// This struct is a convenience wrapper
/// around a russh client
/// that handles the input/output event loop
pub struct Session {
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
            let response = resolver.lookup_ip(server.host.as_str()).await;
            response
                .expect("DNS lookup failed")
                .iter()
                .next()
                .expect("No IP addresses returned")
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

        Ok(Self { session })
    }

    async fn shell(&mut self) -> anyhow::Result<()> {
        // We're using `crossterm` to put the terminal into raw mode, so that we can
        // display the output of interactive applications correctly
        enable_raw_mode()?;

        let mut channel = self.session.channel_open_session().await?;

        let (mut w, mut h) = size()?;
        // Request an interactive PTY from the server
        channel
            .request_pty(
                true,
                &env::var("TERM").unwrap_or("xterm".into()),
                w as u32,
                h as u32,
                0,
                0,
                &[], // ideally you want to pass the actual terminal modes here
            )
            .await?;
        channel.request_shell(true).await?;

        let mut stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut buf = vec![0; 1024];

        loop {
            let (new_w, new_h) = size()?;
            if (w, h) != (new_w, new_h) {
                w = new_w;
                h = new_h;
                channel
                    .window_change(new_w as u32, new_h as u32, 0, 0)
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
                            // channel.eof().await?;
                            // break;

                            disable_raw_mode()?;
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                },
            }
        }

        if is_raw_mode_enabled()? {
            disable_raw_mode()?;
        }

        Ok(())
    }

    async fn close(&mut self) -> anyhow::Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
