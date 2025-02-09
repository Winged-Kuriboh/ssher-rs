use clap::Command;
use cmd::{add_server, connect_server, list_servers, remove_server, rename_server, version};
use config::{load_config, save_config};
use prompt::{server_form_prompt, servers_select_prompt};
mod cmd;
mod colord_print;
mod config;
mod model;
mod prompt;

fn main() {
    let cmd = Command::new("ssher")
        .about("ssher is an easy-to-use command line tool for connecting to remote servers.")
        .subcommand(Command::new("version").alias("v").about("Show version"))
        .subcommand(Command::new("add").about("Add a new server"))
        .subcommand(Command::new("list").alias("ls").about("List all servers"))
        .subcommand(Command::new("remove").alias("rm").about("Remove a server"))
        .subcommand(Command::new("rename").alias("rn").about("Rename a server"));

    let mut config = load_config();

    match cmd.get_matches().subcommand() {
        Some(("version", _)) => version(),
        Some(("add", _)) => add_server(&mut config),
        Some(("list", _)) => list_servers(&config),
        Some(("remove", _)) => remove_server(&mut config),
        Some(("rename", _)) => rename_server(&mut config),
        _ => connect_server(&mut config),
    }
}
