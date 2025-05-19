#![allow(dead_code)]
use crate::{
    cmd::{
        add_server, connect_server, edit_server, list_servers, remove_server, rename_server,
        version,import_servers,
    },
    common::{print_completions, server_completer, servers_len},
};
use clap::{
    Args, CommandFactory, Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};
use clap_complete::{ArgValueCompleter, CompleteEnv, Shell};

#[derive(Debug, Parser)]
#[command(
    name = "ssher",
    about = "ssher is an easy-to-use command line tool for connecting to remote servers.",
    args_conflicts_with_subcommands = true,
    styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Cyan.on_default())
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<SubCommands>,

    #[arg(
        short,
        long,
        help = "Server name",
        add = ArgValueCompleter::new(server_completer),
    )]
    server: Option<String>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(
        name = "version",
        about = "Show version",
        visible_alias = "v",
        disable_help_flag = true
    )]
    Version,
    #[command(
        name = "completion",
        about = "Generate shell completion script",
        disable_help_flag = true
    )]
    Completion {
        #[command(subcommand)]
        command: Option<CompletionSubCommands>,
    },
    #[command(name = "add", about = "Add a new server", disable_help_flag = true)]
    Add,
    #[command(
        name = "list",
        about = "List all servers",
        visible_alias = "ls",
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
        visible_alias = "rm",
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
        name = "import",
        about = "Import servers from ~/.ssh/config",
        allow_missing_positional = true,
        disable_help_flag = true
    )]
    Import,
}

#[derive(Debug, Args)]
struct ServerArgs {
    #[arg(add = ArgValueCompleter::new(server_completer), num_args = ..=1)]
    name: Option<String>,
}

#[derive(Debug, Args)]
struct ServersArgs {
    #[arg(add = ArgValueCompleter::new(server_completer), num_args = ..=servers_len())]
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
    #[command(name = "elvish", about = "Generate elvish completion script")]
    Elvish,
}

impl Cli {
    pub(crate) fn new() -> Self {
        CompleteEnv::with_factory(Cli::command).complete();
        Self::parse()
    }

    pub(crate) async fn run(&self) -> anyhow::Result<()> {
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
                    Some(CompletionSubCommands::Elvish) => Shell::Elvish,
                    None => {
                        anyhow::bail!("😿 Please specify a shell(bash, zsh, fish, powershell)")
                    }
                };
                print_completions(shell, &mut Cli::command())?;
            }
            Some(SubCommands::Add) => {
                add_server()?;
            }
            Some(SubCommands::List) => {
                list_servers()?;
            }
            Some(SubCommands::Edit(server)) => {
                let server = server.name.clone().unwrap_or_default();
                edit_server(server)?;
            }
            Some(SubCommands::Remove(servers)) => {
                remove_server(servers.names.clone())?;
            }
            Some(SubCommands::Rename(server)) => {
                let server = server.name.clone().unwrap_or_default();
                rename_server(server)?;
            }
            Some(SubCommands::Import) => {
               import_servers()?;
            }
            None => {
                let server = self.server.clone().unwrap_or_default();
                connect_server(server).await?;
            }
        }

        Ok(())
    }
}
