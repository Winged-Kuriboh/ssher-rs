use cli::Cli;
mod cli;
mod cmd;
mod colord_print;
mod config;
mod model;
mod prompt;

fn main() {
    Cli::new().run();
}
