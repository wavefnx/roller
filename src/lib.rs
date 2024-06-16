const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_API_ENDPOINT: &str = "https://tracker-api-gdesfolyga-uw.a.run.app";

mod client;
pub use client::Client;

mod network;
pub use network::Network;

mod tui;
pub use tui::Tui;

mod config;
pub use config::Config;

mod terminal;
pub use terminal::Terminal;
