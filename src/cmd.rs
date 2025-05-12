use crate::{
    colord_print::{green, yellow},
    config::{load_config, save_config},
    prompt::{
        add_server_form_prompt, edit_server_form_prompt, rename_server_prompt,
        servers_select_prompt, yesno_select_prompt,
    },
    ssh,
};
use anyhow::Ok;
use std::vec;
use tabled::{Table, settings::Style};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn version() {
    green(format!("😸 Version: v{}", VERSION));
}

pub(crate) fn list_servers() -> anyhow::Result<()> {
    let config = load_config()?;

    if config.servers.is_empty() {
        yellow("😿 No servers found");
    } else {
        let table = Table::new(&config.servers)
            .with(Style::modern_rounded())
            .to_string();

        println!("{table}")
    }
    Ok(())
}

pub(crate) fn remove_server(servers: Vec<String>) -> anyhow::Result<()> {
    let mut config = load_config()?;

    let servers = if servers.is_empty() {
        match servers_select_prompt(&config.servers) {
            Some(s) => vec![s.name],
            None => return Ok(()),
        }
    } else {
        let mut servers_removed = vec![];
        for name in servers.clone() {
            if !config.servers.iter().any(|s| s.name == name) {
                yellow(format!("😿 No server <{}> found", &name));
            } else {
                servers_removed.push(name);
            }
        }
        servers_removed
    };

    if servers.is_empty() {
        return Ok(());
    }

    let label = if servers.len() > 1 {
        "Are you sure you want to remove these servers?"
    } else {
        "Are you sure you want to remove this server?"
    };

    if yesno_select_prompt(label)? {
        let mut server_removed = vec![];
        for server in servers {
            if let Some(index) = config.servers.iter().position(|s| s.name == server) {
                config.servers.remove(index);
                server_removed.push(server);
            };
        }
        save_config(&config)?;
        green(format!("😺 Server {} removed.", server_removed.join(", ")));
    }

    Ok(())
}

pub(crate) fn add_server() -> anyhow::Result<()> {
    let mut config = load_config()?;

    if let Some(server) = add_server_form_prompt(&config)? {
        let server_name = server.name.clone();

        config.servers.push(server);
        save_config(&config)?;

        green(format!("😺 Server {} added.", server_name));
    }

    Ok(())
}

pub(crate) fn edit_server(server: String) -> anyhow::Result<()> {
    let mut config = load_config()?;

    let server = match config.servers.iter().find(|s| s.name == server) {
        Some(s) => s.clone(),
        None => {
            if let Some(s) = servers_select_prompt(&config.servers) {
                s
            } else {
                return Ok(());
            }
        }
    };

    if let Some(new_server) = edit_server_form_prompt(&config, &server)? {
        if let Some(index) = config.servers.iter().position(|s| s.name == server.name) {
            config.servers[index] = new_server;
            save_config(&config)?;
            green(format!("😺 Server {} updated.", server.name));
        };
    }

    Ok(())
}

pub(crate) fn rename_server(server: String) -> anyhow::Result<()> {
    let mut config = load_config()?;

    let server = match config.servers.iter().find(|s| s.name == server) {
        Some(s) => s.clone(),
        None => {
            if let Some(s) = servers_select_prompt(&config.servers) {
                s
            } else {
                return Ok(());
            }
        }
    };

    let new_name = rename_server_prompt(&config, &server)?;
    if server.name != new_name {
        for s in &mut config.servers {
            if s.name == server.name {
                s.name = new_name.clone();
            }
        }
        save_config(&config)?;

        green(format!(
            "😺 Server {} renamed to {}.",
            server.name, new_name
        ));
    }

    Ok(())
}

pub(crate) async fn connect_server(server: String) -> anyhow::Result<()> {
    let mut config = load_config()?;

    let server = match config.servers.iter().find(|s| s.name == server) {
        Some(s) => s.clone(),
        None => {
            if let Some(s) = servers_select_prompt(&config.servers) {
                s
            } else {
                return Ok(());
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
        save_config(&config)?;
    }

    ssh::exec(server).await?;

    Ok(())
}
