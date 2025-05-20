use ssher::{cli::Cli, colord_print::red};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = Cli::new().run().await {
        red(e.to_string());
    }

    std::process::exit(0);
}
