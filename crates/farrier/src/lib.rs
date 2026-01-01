mod error;
mod marketplace;
mod plugin;

pub use error::Error;
pub use marketplace::{parse_marketplace, Marketplace, PluginEntry};
pub use plugin::{parse_plugin_manifest, PluginManifest};

pub type Result<T> = std::result::Result<T, Error>;
