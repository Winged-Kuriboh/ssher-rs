use cli::Cli;
use colord_print::red;
mod cli;
mod cli_builder;
mod cmd;
mod colord_print;
mod common;
mod config;
mod model;
mod prompt;
mod ssh;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // builder version
    // if let Err(e) = cli_builder::run().await {
    //     red(e.to_string().as_str());
    // }

    // derive version
    if let Err(e) = Cli::new().run().await {
        red(e.to_string().as_str());
    }

    std::process::exit(0);
}
