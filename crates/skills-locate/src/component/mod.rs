mod agent;
mod command;
mod hook;
mod manifest;
mod mcp;
mod npm;
mod python;
mod skill;

pub use agent::{parse_agent_descriptor, AgentDescriptor};
pub use command::{parse_command_descriptor, CommandDescriptor};
#[allow(unused_imports)]
pub use hook::{parse_hooks_json, HookAction, HookEvent, HookGroup, HooksConfig};
pub use manifest::{parse_manifest, ManifestConfig};
pub use mcp::{parse_mcp_json, McpServer};
pub use npm::detect_npm_mcp;
pub use python::detect_python_mcp;
pub use skill::parse_skill_descriptor;
