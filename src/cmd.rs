use crate::{
    colord_print::{green, red, yellow},
    config::{load_config, save_config},
    prompt::{
        add_server_form_prompt, edit_server_form_prompt, rename_server_prompt,
        servers_select_prompt, yesno_select_prompt,
    },
};
use base64::{engine::general_purpose, Engine};
use ssh2::{PtyModes, Session};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    path::Path,
    vec,
};
use tabled::{settings::Style, Table};
use termion::{async_stdin, raw::IntoRawMode, terminal_size};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::AsyncResolver;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn version() {
    green(format!("😸 Version: v{}", VERSION).as_str());
}

pub(crate) fn list_servers() {
    let config = load_config();

    if config.servers.is_empty() {
        yellow("😿 No servers found");
    } else {
        let table = Table::new(&config.servers)
            .with(Style::modern_rounded())
            .to_string();

        println!("{table}")
    }
}

pub(crate) fn remove_server(servers: Vec<String>) {
    let mut config = load_config();

    let servers = if servers.is_empty() {
        match servers_select_prompt(&config.servers) {
            Some(s) => vec![s.name],
            None => return,
        }
    } else {
        let mut servers_removed = vec![];
        for name in servers.clone() {
            if !config.servers.iter().any(|s| s.name == name) {
                yellow(format!("😿 No server <{}> found", &name).as_str());
            } else {
                servers_removed.push(name);
            }
        }
        servers_removed
    };

    if servers.is_empty() {
        return;
    }

    let label = if servers.len() > 1 {
        "Are you sure you want to remove these servers?"
    } else {
        "Are you sure you want to remove this server?"
    };

    if yesno_select_prompt(label) {
        let mut server_removed = vec![];
        for server in servers {
            let index = config
                .servers
                .iter()
                .position(|s| s.name == server)
                .unwrap();

            config.servers.remove(index);
            server_removed.push(server);
        }
        save_config(&config);
        green(format!("😺 Server {} removed.", server_removed.join(", ")).as_str());
    }
}

pub(crate) fn add_server() {
    let mut config = load_config();

    if let Some(server) = add_server_form_prompt(&config) {
        let server_name = server.name.clone();

        config.servers.push(server);
        save_config(&config);

        green(format!("😺 Server {} added.", server_name).as_str());
    }
}

pub(crate) fn edit_server(server: String) {
    let mut config = load_config();

    let server = match config.servers.iter().find(|s| s.name == server) {
        Some(s) => s.clone(),
        None => {
            if let Some(s) = servers_select_prompt(&config.servers) {
                s
            } else {
                return;
            }
        }
    };

    if let Some(new_server) = edit_server_form_prompt(&config, &server) {
        let index = config
            .servers
            .iter()
            .position(|s| s.name == server.name)
            .unwrap();

        config.servers[index] = new_server;
        save_config(&config);
        green(format!("😺 Server {} updated.", server.name).as_str());
    }
}

pub(crate) fn rename_server(server: String) {
    let mut config = load_config();

    let server = match config.servers.iter().find(|s| s.name == server) {
        Some(s) => s.clone(),
        None => {
            if let Some(s) = servers_select_prompt(&config.servers) {
                s
            } else {
                return;
            }
        }
    };

    let new_name = rename_server_prompt(&config, &server);
    if server.name != new_name {
        for s in &mut config.servers {
            if s.name == server.name {
                s.name = new_name.clone();
            }
        }
        save_config(&config);

        green(format!("😺 Server {} renamed to {}.", server.name, new_name).as_str());
    }
}

pub(crate) fn connect_server(server: String) {
    let mut config = load_config();

    let server = match config.servers.iter().find(|s| s.name == server) {
        Some(s) => s.clone(),
        None => {
            if let Some(s) = servers_select_prompt(&config.servers) {
                s
            } else {
                return;
            }
        }
    };

    // If the server is not marked as current, mark it as current,
    // and unmark all others.
    if server.current.is_none_or(|c| !c) {
        for s in &mut config.servers {
            if s.name == server.name {
                s.current = Some(true);
            } else {
                s.current = None;
            }
        }
        save_config(&config);
    }

    let host = if server.host.parse::<std::net::IpAddr>().is_ok() {
        server.host.clone()
    } else {
        let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        let response = resolver.lookup_ip(server.host.as_str());
        let ip = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(response)
            .expect("DNS lookup failed")
            .iter()
            .next()
            .expect("No IP addresses returned");
        ip.to_string()
    };

    let tcp = TcpStream::connect(format!("{}:{}", host, server.port)).expect("Failed to connect");
    tcp.set_nodelay(true).unwrap();

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let mut userauth = false;
    if let Some(ref password) = server.password {
        if !password.is_empty() {
            let password = general_purpose::STANDARD.decode(password).unwrap();
            sess.userauth_password(&server.user, &String::from_utf8(password).unwrap())
                .unwrap();
            userauth = true;
        }
    }
    if !userauth {
        if let Some(ref identity_file) = server.identity_file {
            let expanded_path = shellexpand::tilde(identity_file).into_owned();
            sess.userauth_pubkey_file(&server.user, None, Path::new(&expanded_path), None)
                .unwrap();
        }
    }

    if sess.authenticated() {
        let mut channel = sess.channel_session().unwrap();

        let (mut cols, mut rows) = terminal_size().unwrap();
        channel
            .request_pty(
                "xterm-256color",
                Some(PtyModes::new()),
                Some((cols as u32, rows as u32, 0, 0)),
            )
            .unwrap();
        channel
            .handle_extended_data(ssh2::ExtendedData::Merge)
            .unwrap();
        channel.shell().unwrap();

        let stdout = io::stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let mut stdin = async_stdin();

        let mut buff_in = Vec::new();
        while !channel.eof() {
            // monite terminal size changes
            let (new_cols, new_rows) = terminal_size().unwrap();
            if cols != new_cols || rows != new_rows {
                cols = new_cols;
                rows = new_rows;
                channel
                    .request_pty_size(new_cols as u32, new_rows as u32, None, None)
                    .unwrap();
            }

            let bytes_available = channel.read_window().available;
            if bytes_available > 0 {
                let mut buffer = vec![0; bytes_available as usize];
                channel.read_exact(&mut buffer).unwrap();
                stdout.write_all(&buffer).unwrap();
                stdout.flush().unwrap();
            }
            stdin.read_to_end(&mut buff_in).unwrap();
            #[allow(clippy::unused_io_amount)]
            channel.write(&buff_in).unwrap();
            buff_in.clear();

            // Unsure of best practice, but this feels responsive and avoids pegging the CPU
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        channel.wait_close().unwrap();
    } else {
        red("😿 Authentication failed.");
    }
}
