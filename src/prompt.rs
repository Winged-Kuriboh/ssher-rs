#![allow(dead_code)]
use crate::{
    colord_print::yellow,
    model::{Config, Server},
};
use base64::{engine::general_purpose, Engine};
use dialoguer::{
    console::{style, Style},
    theme::ColorfulTheme,
    Confirm, Input, Password, Select,
};

pub(crate) fn default_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new().for_stderr().cyan(),
        prompt_style: Style::new().for_stderr().bold(),
        prompt_prefix: style("🐱".to_string()).for_stderr().yellow(),
        prompt_suffix: style("".to_string()).blue(),
        success_prefix: style("✔".to_string()).for_stderr().cyan(),
        success_suffix: style("".to_string()).for_stderr().black().bright(),
        error_prefix: style("✘".to_string()).for_stderr().red(),
        error_style: Style::new().for_stderr().red(),
        hint_style: Style::new().for_stderr().black().bright(),
        values_style: Style::new().for_stderr().cyan(),
        active_item_style: Style::new().for_stderr().cyan().underlined(),
        inactive_item_style: Style::new().for_stderr(),
        active_item_prefix: style("➤".to_string()).for_stderr().cyan(),
        inactive_item_prefix: style(" ".to_string()).for_stderr(),
        checked_item_prefix: style("✔".to_string()).for_stderr().cyan(),
        unchecked_item_prefix: style("⬚".to_string()).for_stderr().magenta(),
        picked_item_prefix: style("➤".to_string()).for_stderr().cyan(),
        unpicked_item_prefix: style(" ".to_string()).for_stderr(),
    }
}

pub(crate) fn servers_select_prompt(server: &[Server]) -> Option<Server> {
    let mut selections: Vec<String> = server
        .iter()
        // .map(|s| format!("{}@{}:{}", s.user, s.host, s.port))
        .map(|s| {
            if let Some(true) = s.current {
                format!("✦ {}", s.name)
            } else {
                format!("  {}", s.name)
            }
        })
        .collect();

    selections.push("✗ Exit".to_string());

    let selection = Select::with_theme(&default_theme())
        .with_prompt("Select a server:")
        .default(0)
        .report(false)
        .items(&selections)
        .interact()
        .ok()?;

    if selection == selections.len() - 1 {
        return None;
    }

    Some(server[selection].clone())
}

pub(crate) fn server_form_prompt(config: &Config) -> Option<Server> {
    let name: String = Input::with_theme(&default_theme())
        .with_prompt("Name(*):")
        .validate_with(|input: &String| {
            if config.servers.iter().any(|s| s.name == *input) {
                Err(format!("😾 Name {} already exists.", input))
            } else {
                Ok(())
            }
        })
        .allow_empty(false)
        .interact_text()
        .unwrap();

    let host: String = Input::with_theme(&default_theme())
        .with_prompt("Host(*):")
        .allow_empty(false)
        .interact_text()
        .unwrap();

    let port: u16 = Input::with_theme(&default_theme())
        .with_prompt("Port(*):")
        .default(22)
        .with_initial_text("22")
        .show_default(false)
        .allow_empty(false)
        .interact_text()
        .unwrap();

    let user: String = Input::with_theme(&default_theme())
        .with_prompt("User(*):")
        .default("root".to_string())
        .with_initial_text("root")
        .show_default(false)
        .allow_empty(false)
        .interact_text()
        .unwrap();

    let password: String = Password::with_theme(&default_theme())
        .with_prompt("Password:")
        .allow_empty_password(true)
        .interact()
        .unwrap();

    let identity_file: String = Input::with_theme(&default_theme())
        .with_prompt("IdentityFile:")
        .with_initial_text("~/.ssh/id_rsa")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    Some(Server {
        name,
        host,
        port,
        user,
        password: if password.is_empty() {
            None
        } else {
            // base64 encode
            Some(general_purpose::STANDARD.encode(password))
        },
        identity_file: if identity_file.is_empty() {
            None
        } else {
            Some(identity_file)
        },
        current: None,
    })
}

pub(crate) fn confirm_prompt(prompt: &str) -> bool {
    Confirm::with_theme(&default_theme())
        .with_prompt(prompt)
        .default(false)
        .report(false)
        .interact()
        .unwrap()
}

pub(crate) fn yesno_select_prompt(prompt: &str) -> bool {
    let selections = vec!["No", "Yes"];
    let selection = Select::with_theme(&default_theme())
        .with_prompt(prompt)
        .default(0)
        .report(false)
        .items(&selections)
        .interact()
        .ok()
        .unwrap();

    selection == 1
}

pub(crate) fn rename_server_prompt(config: &Config, server: &Server) -> String {
    Input::with_theme(&default_theme())
        .with_prompt("New name(*):")
        .validate_with(|input: &String| {
            if *input == server.name {
                yellow("😺 Name not changed.");
                return Ok(());
            }

            if config.servers.iter().any(|s| s.name == *input) {
                Err(format!("😾 Name {} already exists.", input))
            } else {
                Ok(())
            }
        })
        .report(false)
        .allow_empty(false)
        .interact_text()
        .unwrap()
}
