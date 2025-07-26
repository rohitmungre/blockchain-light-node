mod config;
mod network;
mod header_store;
mod light_client;

use anyhow::Result;
use config::Settings;
use light_client::LightClient;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let settings = Settings::parse();
    let mut client = LightClient::new(&settings).await?;
    println!("Starting header sync…");
    tokio::select! {
        res = client.sync_headers() => res?,
        _ = signal::ctrl_c() => println!("Shutting down…"),
    }
    Ok(())
}
