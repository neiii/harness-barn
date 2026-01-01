mod error;
mod marketplace;

pub use error::Error;
pub use marketplace::{parse_marketplace, Marketplace, PluginEntry};

pub type Result<T> = std::result::Result<T, Error>;
