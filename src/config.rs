use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="light-node")]
pub struct Settings {
    /// Multiaddr of bootstrap peer
    #[arg(long, default_value = "/ip4/127.0.0.1/tcp/4001")]
    pub peer_addr: String,

    /// Maximum number of headers to keep
    #[arg(long, default_value_t = 1000)]
    pub max_headers: usize,
}
