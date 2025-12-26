#![doc = include_str!("../README.md")]
//!
//! ## Modules
//!
//! - [`harness`] - Harness discovery and path resolution
//! - [`mcp`] - MCP server type definitions
//! - [`types`] - Core type definitions
//! - [`error`] - Error types

pub mod error;
pub mod harness;
pub mod mcp;
pub mod platform;
pub mod types;

pub use error::{Error, Result};
pub use harness::Harness;
pub use mcp::{
    HttpMcpServer, McpCapabilities, McpServer, OAuthConfig, SseMcpServer, StdioMcpServer,
};
pub use types::{
    ConfigResource, DirectoryResource, DirectoryStructure, EnvValue, FileFormat, HarnessKind,
    PathType, Scope,
};
