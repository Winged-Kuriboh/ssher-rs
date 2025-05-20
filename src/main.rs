use colord_print::red;
mod cli;
mod cmd;
mod colord_print;
mod common;
mod config;
mod endec;
mod model;
mod prompt;
mod ssh;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = cli::Cli::new().run().await {
        red(e.to_string());
    }

    std::process::exit(0);
}
