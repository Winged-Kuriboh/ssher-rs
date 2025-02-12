use crate::config::load_config;
use clap::{
    builder::{PossibleValue, StyledStr},
    Command,
};
use clap_complete::{generate, CompletionCandidate, Generator};
use std::io;

pub(crate) fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub(crate) fn server_possible_values() -> Vec<PossibleValue> {
    let config = load_config();

    config
        .servers
        .iter()
        .map(|s| {
            PossibleValue::new(s.name.clone()).help(format!(
                "[{}] {}@{}:{}",
                if s.current.unwrap_or_default() {
                    "✲"
                } else {
                    " "
                },
                s.user,
                s.host,
                s.port
            ))
        })
        .collect()
}

pub(crate) fn servers_len() -> usize {
    load_config().servers.len()
}

pub(crate) fn server_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let config = load_config();

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
