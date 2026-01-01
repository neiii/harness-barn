//! MCP server descriptor types and parsing.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// MCP server descriptor from .mcp.json files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct McpDescriptor {
    /// Server name (key from the JSON object).
    #[serde(skip)]
    pub name: String,
    /// Command to execute the server.
    pub command: String,
    /// Arguments to pass to the command.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Environment variables for the server.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct McpServerEntry {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}

/// Parse a .mcp.json file content into a list of MCP descriptors.
pub fn parse_mcp_json(content: &str) -> Result<Vec<McpDescriptor>> {
    let map: HashMap<String, McpServerEntry> =
        serde_json::from_str(content).map_err(Error::JsonParse)?;

    Ok(map
        .into_iter()
        .map(|(name, entry)| McpDescriptor {
            name,
            command: entry.command,
            args: entry.args,
            env: entry.env,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_descriptor_serde_roundtrip() {
        let desc = McpDescriptor {
            name: String::new(),
            command: "npx".to_string(),
            args: vec!["-y".to_string(), "server".to_string()],
            env: HashMap::from([("NODE_ENV".to_string(), "production".to_string())]),
        };
        let json = serde_json::to_string(&desc).unwrap();
        assert!(json.contains("npx"));
        assert!(json.contains("NODE_ENV"));
    }

    #[test]
    fn parse_single_server() {
        let content = r#"{
            "my-server": {
                "command": "node",
                "args": ["server.js"],
                "env": {"PORT": "3000"}
            }
        }"#;
        let servers = parse_mcp_json(content).unwrap();
        assert_eq!(servers.len(), 1);
        let server = &servers[0];
        assert_eq!(server.name, "my-server");
        assert_eq!(server.command, "node");
        assert_eq!(server.args, vec!["server.js"]);
        assert_eq!(server.env.get("PORT"), Some(&"3000".to_string()));
    }

    #[test]
    fn parse_multiple_servers() {
        let content = r#"{
            "server-a": {"command": "cmd-a"},
            "server-b": {"command": "cmd-b", "args": ["--flag"]}
        }"#;
        let servers = parse_mcp_json(content).unwrap();
        assert_eq!(servers.len(), 2);
    }

    #[test]
    fn parse_empty_mcp_json() {
        let content = "{}";
        let servers = parse_mcp_json(content).unwrap();
        assert!(servers.is_empty());
    }

    #[test]
    fn parse_minimal_server() {
        let content = r#"{"minimal": {"command": "echo"}}"#;
        let servers = parse_mcp_json(content).unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].command, "echo");
        assert!(servers[0].args.is_empty());
        assert!(servers[0].env.is_empty());
    }

    #[test]
    fn parse_invalid_json_returns_error() {
        let content = "not json";
        assert!(parse_mcp_json(content).is_err());
    }
}
