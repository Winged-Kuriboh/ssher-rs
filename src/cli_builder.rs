#![allow(dead_code)]
use crate::{
    cmd::{
        add_server, connect_server, edit_server, list_servers, remove_server, rename_server,
        version,
    },
    colord_print::red,
    command::{print_completions, server_completer, servers_len},
};
use clap::{
    builder::{styling::AnsiColor, Styles},
    Arg, Command,
};
use clap_complete::{ArgValueCompleter, CompleteEnv, Shell};

fn version_cmd() -> Command {
    Command::new("version")
        .about("Show version")
        .visible_alias("v")
        .disable_help_flag(true)
}

fn completion_cmd() -> Command {
    Command::new("completion")
        .about("Generate shell completion script")
        .subcommands([
            Command::new("bash").about("Generate bash completion script"),
            Command::new("zsh").about("Generate zsh completion script"),
            Command::new("fish").about("Generate fish completion script"),
            Command::new("powershell").about("Generate powershell completion script"),
        ])
        .disable_help_flag(true)
}

fn add_server_cmd() -> Command {
    Command::new("add")
        .about("Add a new server")
        .disable_help_subcommand(true)
}

fn list_servers_cmd() -> Command {
    Command::new("list")
        .about("List all servers")
        .visible_alias("ls")
        .disable_help_flag(true)
}

fn edit_server_cmd() -> Command {
    Command::new("edit")
        .about("Edit a server")
        .allow_missing_positional(true)
        .arg(
            Arg::new("server")
                .add(ArgValueCompleter::new(server_completer))
                .num_args(..=1),
        )
        .disable_help_flag(true)
}

fn remove_server_cmd() -> Command {
    Command::new("remove")
        .about("Remove a server or servers")
        .visible_alias("rm")
        .allow_missing_positional(true)
        .arg(clap::Arg::new("servers").num_args(..=servers_len()))
        .disable_help_flag(true)
}

fn rename_server_cmd() -> Command {
    Command::new("rename")
        .about("Rename a server")
        .allow_missing_positional(true)
        .arg(
            Arg::new("server")
                .add(ArgValueCompleter::new(server_completer))
                .num_args(..=1),
        )
        .disable_help_flag(true)
}

fn build_cli() -> Command {
    Command::new("ssher")
        .about("ssher is an easy-to-use command line tool for connecting to remote servers.")
        .arg(
            Arg::new("server")
                .long("server")
                .short('s')
                .add(ArgValueCompleter::new(server_completer))
                .help("Server name"),
        )
        .subcommands([
            version_cmd(),
            completion_cmd(),
            add_server_cmd(),
            list_servers_cmd(),
            edit_server_cmd(),
            remove_server_cmd(),
            rename_server_cmd(),
        ])
        .styles(
            Styles::styled()
                .header(AnsiColor::Yellow.on_default())
                .usage(AnsiColor::Yellow.on_default())
                .literal(AnsiColor::Cyan.on_default())
                .placeholder(AnsiColor::Cyan.on_default()),
        )
        .args_conflicts_with_subcommands(true)
}

pub(crate) fn run() {
    CompleteEnv::with_factory(build_cli).complete();

    let cli = build_cli();

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("version", _)) => {
            version();
        }
        Some(("completion", sm)) => {
            let shell = match sm.subcommand() {
                Some(("bash", _)) => Shell::Bash,
                Some(("zsh", _)) => Shell::Zsh,
                Some(("fish", _)) => Shell::Fish,
                Some(("powershell", _)) => Shell::PowerShell,
                _ => {
                    red("😿 Please specify a shell(bash, zsh, fish, powershell)");
                    return;
                }
            };
            print_completions(shell, &mut build_cli());
        }
        Some(("add", _)) => {
            add_server();
        }
        Some(("list", _)) => {
            list_servers();
        }
        Some(("edit", arg_matches)) => {
            let server = match arg_matches.get_one::<String>("server") {
                Some(s) => s.to_string(),
                None => String::new(),
            };
            edit_server(server);
        }
        Some(("remove", arg_matches)) => {
            let mut servers: Vec<String> = arg_matches
                .get_many::<String>("servers")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();

            if servers.len() > 1 {
                servers.dedup();
            }

            remove_server(servers);
        }
        Some(("rename", arg_matches)) => {
            let server = match arg_matches.get_one::<String>("server") {
                Some(s) => s.to_string(),
                None => String::new(),
            };
            rename_server(server);
        }
        Some((_, _)) => {}
        None => {
            let server = match matches.get_one::<String>("server") {
                Some(s) => s.to_string(),
                None => String::new(),
            };
            connect_server(server);
        }
    }
}
