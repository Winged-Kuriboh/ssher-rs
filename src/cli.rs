use crate::{
    cmd::{
        add_server, connect_server, edit_server, list_servers, remove_server, rename_server,
        version,
    },
    colord_print::red,
};
use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(Debug, Parser)]
#[command(name= "ssher", about = "ssher is an easy-to-use command line tool for connecting to remote servers.", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(name = "version", about = "Show version", alias = "v")]
    Version,
    #[command(name = "add", about = "Add a new server")]
    Add,
    #[command(name = "list", about = "List all servers", alias = "ls")]
    List,
    #[command(name = "edit", about = "Edit a server")]
    Edit(ServerArgs),
    #[command(name = "remove", about = "Remove a server or servers", alias = "rm")]
    Remove(ServersArgs),
    #[command(name = "rename", about = "Rename a server")]
    Rename(ServerArgs),
    #[command(name = "completion", about = "Generate shell completion script")]
    Completion {
        #[command(subcommand)]
        command: Option<CompletionSubCommands>,
    },
}

#[derive(Debug, Args)]
struct ServerArgs {
    name: Option<String>,
}

#[derive(Debug, Args)]
struct ServersArgs {
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
                let server = match server.name.clone() {
                    Some(server) => server,
                    None => String::new(),
                };
                edit_server(server);
            }
            Some(SubCommands::Remove(servers)) => {
                remove_server(servers.names.clone());
            }
            Some(SubCommands::Rename(server)) => {
                let server = match server.name.clone() {
                    Some(server) => server,
                    None => String::new(),
                };
                rename_server(server);
            }
            None => connect_server(),
        }
    }
}
