//! Skills discovery and fetching for AI coding agents.

mod component;
mod detect;
mod discovery;
mod error;
mod fetch;
mod github;
mod marketplace;
mod registry;
mod types;

pub use component::{
    AgentDescriptor, CommandDescriptor, HooksConfig, McpServer, detect_npm_mcp,
    detect_python_mcp, parse_agent_descriptor, parse_command_descriptor, parse_manifest,
    parse_mcp_json, parse_skill_descriptor, ManifestConfig,
};
pub use detect::{detect_mcp_from_files, DetectedMcp, DetectionConfidence, DetectionSource};
pub use discovery::{discover_all, discover_from_source, discover_plugins};
pub use error::{Error, Result};
pub use fetch::{extract_file, fetch_bytes, fetch_json, list_files};
pub use github::GitHubRef;
pub use marketplace::{Marketplace, MarketplaceEntry};
pub use registry::{PackageEntry, RegistryClient, RemoteEntry, ServerEntry};
pub use types::{DiscoveryResult, PluginDescriptor, PluginSource, SkillDescriptor};
