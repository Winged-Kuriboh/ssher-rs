#![allow(dead_code)]
use crate::{
    cmd::{
        add_server, connect_server, edit_server, list_servers, remove_server, rename_server,
        version,
    },
    colord_print::red,
    command::{print_completions, server_completer, server_possible_values},
};
use clap::{Args, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{ArgValueCompleter, CompleteEnv, Shell};

#[derive(Debug, Parser)]
#[command(
    name = "ssher",
    about = "ssher is an easy-to-use command line tool for connecting to remote servers.",
    args_conflicts_with_subcommands = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<SubCommands>,

    #[arg(short, long, help = "Server name")]
    #[arg(add = ArgValueCompleter::new(server_completer), value_parser = server_possible_values(), num_args = 1)]
    server: Option<String>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(
        name = "version",
        about = "Show version",
        alias = "v",
        disable_help_flag = true
    )]
    Version,
    #[command(name = "add", about = "Add a new server")]
    Add,
    #[command(
        name = "list",
        about = "List all servers",
        alias = "ls",
        disable_help_flag = true
    )]
    List,
    #[command(
        name = "edit",
        about = "Edit a server",
        allow_missing_positional = true,
        disable_help_flag = true
    )]
    Edit(ServerArgs),
    #[command(
        name = "remove",
        about = "Remove a server or servers",
        alias = "rm",
        allow_missing_positional = true,
        disable_help_flag = true
    )]
    Remove(ServersArgs),
    #[command(
        name = "rename",
        about = "Rename a server",
        allow_missing_positional = true,
        disable_help_flag = true
    )]
    Rename(ServerArgs),
    #[command(
        name = "completion",
        about = "Generate shell completion script",
        disable_help_flag = true
    )]
    Completion {
        #[command(subcommand)]
        command: Option<CompletionSubCommands>,
    },
}

#[derive(Debug, Args)]
struct ServerArgs {
    #[arg(value_hint = ValueHint::Other, add = ArgValueCompleter::new(server_completer), value_parser = server_possible_values(), num_args = ..=1)]
    name: Option<String>,
}

#[derive(Debug, Args)]
struct ServersArgs {
    #[arg(value_hint = ValueHint::Other, add = ArgValueCompleter::new(server_completer), value_parser = server_possible_values(), num_args = ..=server_possible_values().len())]
    names: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum CompletionSubCommands {
    #[command(name = "bash", about = "Generate bash completion script")]
    Bash,
    #[command(name = "zsh", about = "Generate zsh completion script")]
    Zsh,
    #[command(name = "fish", about = "Generate fish completion script")]
    Fish,
    #[command(name = "powershell", about = "Generate powershell completion script")]
    Powershell,
}

impl Cli {
    pub(crate) fn new() -> Self {
        CompleteEnv::with_factory(Self::command).complete();
        Self::parse()
    }

    pub(crate) fn run(&self) {
        match &self.command {
            Some(SubCommands::Version) => {
                version();
            }
            Some(SubCommands::Completion { command }) => {
                let shell = match *command {
                    Some(CompletionSubCommands::Bash) => Shell::Bash,
                    Some(CompletionSubCommands::Zsh) => Shell::Zsh,
                    Some(CompletionSubCommands::Fish) => Shell::Fish,
                    Some(CompletionSubCommands::Powershell) => Shell::PowerShell,
                    None => {
                        red("😿 Please specify a shell(bash, zsh, fish, powershell)");
                        return;
                    }
                };
                print_completions(shell, &mut Cli::command());
            }
            Some(SubCommands::Add) => {
                add_server();
            }
            Some(SubCommands::List) => {
                list_servers();
            }
            Some(SubCommands::Edit(server)) => {
                let server = server.name.clone().unwrap_or_default();
                edit_server(server);
            }
            Some(SubCommands::Remove(servers)) => {
                remove_server(servers.names.clone());
            }
            Some(SubCommands::Rename(server)) => {
                let server = server.name.clone().unwrap_or_default();
                rename_server(server);
            }
            None => {
                let server = self.server.clone().unwrap_or_default();
                connect_server(server);
            }
        }
    }
}
