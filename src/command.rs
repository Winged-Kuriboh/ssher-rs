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

pub(crate) fn server_completer(_current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let config = load_config();

    config
        .servers
        .iter()
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
