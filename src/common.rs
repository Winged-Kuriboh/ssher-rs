use crate::config::load_config;
use clap::{Command, builder::StyledStr};
use clap_complete::{CompletionCandidate, Shell, env::Shells, generate};
use std::io;

pub(crate) fn print_completions(shell: Shell, cmd: &mut Command) -> anyhow::Result<()> {
    generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());

    println!();

    let name = cmd.get_name();
    if let Some(completer) = Shells::builtins().completer(shell.to_string().as_str()) {
        completer.write_registration("COMPLETE", name, name, name, &mut io::stdout())?;
    }

    Ok(())
}

pub(crate) fn servers_len() -> usize {
    load_config().unwrap().servers.len()
}

pub(crate) fn server_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let config = load_config().unwrap();

    let current = current.to_str().unwrap_or_default();

    config
        .servers
        .iter()
        .filter(|s| s.name.contains(current) || s.host.contains(current))
        .map(|s| {
            let help = Some(StyledStr::from(format!(
                "[{}] {}@{}:{}",
                if s.current.unwrap_or_default() {
                    "✲"
                } else {
                    " "
                },
                s.user,
                s.host,
                s.port
            )));

            CompletionCandidate::new(s.name.clone()).help(help)
        })
        .collect()
}
