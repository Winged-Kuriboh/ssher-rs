use cli::Cli;
mod cli;
mod cli_builder;
mod cmd;
mod colord_print;
mod command;
mod config;
mod model;
mod prompt;

fn main() {
    // builder version
    // cli_builder::run()

    // derive version
    Cli::new().run();
}
