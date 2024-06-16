use clap::Parser;
use serde::Serialize;

#[derive(Parser, Debug, Serialize)]
#[clap(version = crate::VERSION, author = "wavefnx @wavefnx")]
/// Terminal interface tracking gas, transactions and data processed by Decentralized Networks.
pub struct Config {
    /// Interval in ms to wait between events.
    /// Increase for lower resource consumption, decrease for more frequent updates.
    #[clap(long, short = 'i', default_value = "100")]
    pub interval_ms: u64,

    /// Change the default API Endpoint by specifying a different URL.
    #[clap(long, default_value = crate::DEFAULT_API_ENDPOINT)]
    pub api_endpoint: String,
}
