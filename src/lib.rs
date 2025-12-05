pub mod io;
pub mod reads;
pub mod alignment;
pub mod clustering;
pub mod consensus;
pub mod variants;
pub mod metrics;
pub mod utils;
pub mod config;

pub use config::Config;
pub use reads::{SequenceRead, Platform};
pub use clustering::Cluster;
pub use metrics::ClusterMetrics;

/// AmpliClust library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type for AmpliClust operations
pub type Result<T> = anyhow::Result<T>;
