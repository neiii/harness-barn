//! Plugin discovery from GitHub repositories.

use crate::component::{
    parse_agent_descriptor, parse_command_descriptor, parse_hooks_json, parse_mcp_json,
    parse_skill_descriptor,
};
use crate::error::{Error, Result};
use crate::fetch::{extract_file, fetch_bytes, list_files};
use crate::github::GitHubRef;
use crate::marketplace::Marketplace;
use crate::types::{PluginDescriptor, PluginSource};

#[derive(Debug, Clone, serde::Deserialize)]
struct PluginJson {
    name: String,
    #[serde(default)]
    description: Option<String>,
}

pub fn discover_plugins(repo_url: &str) -> Result<Vec<PluginDescriptor>> {
    let github_ref = GitHubRef::parse(repo_url)?;
    let archive_url = github_ref.archive_url();
    let archive_bytes = fetch_bytes(&archive_url)?;

    let marketplace_path = find_marketplace_json(&archive_bytes)?;
    let marketplace_content = extract_file(&archive_bytes, &marketplace_path)?;
    let marketplace: Marketplace = serde_json::from_str(&marketplace_content)?;

    let mut plugins = Vec::new();
    let prefix = extract_archive_prefix(&archive_bytes)?;

    for entry in marketplace.plugins {
        let source_str = extract_source_path(&entry.source);
        let plugin_path = resolve_plugin_path(&source_str);

        if let Ok(plugin) = discover_single_plugin(&archive_bytes, &prefix, &plugin_path) {
            plugins.push(plugin);
        }
    }

    Ok(plugins)
}

fn find_marketplace_json(archive: &[u8]) -> Result<String> {
    let candidates = list_files(archive, "marketplace.json")?;

    for path in candidates {
        if path.contains(".claude-plugin/marketplace.json") {
            return Ok(path);
        }
    }

    Err(Error::NotFound(
        ".claude-plugin/marketplace.json".to_string(),
    ))
}

fn extract_archive_prefix(archive: &[u8]) -> Result<String> {
    let files = list_files(archive, "")?;
    if let Some(first) = files.first()
        && let Some(slash_pos) = first.find('/')
    {
        return Ok(first[..=slash_pos].to_string());
    }
    Ok(String::new())
}

fn extract_source_path(source: &PluginSource) -> String {
    match source {
        PluginSource::Relative(path) => path.clone(),
        PluginSource::GitHub { github } => github.clone(),
        PluginSource::Url { url } => url.clone(),
    }
}

fn resolve_plugin_path(source: &str) -> String {
    source.strip_prefix("./").unwrap_or(source).to_string()
}

fn scan_components<T, F>(
    archive: &[u8],
    plugin_prefix: &str,
    subdir: &str,
    suffix: &str,
    parser: F,
) -> Vec<T>
where
    F: Fn(&str) -> Option<T>,
{
    let dir_prefix = format!("{plugin_prefix}{subdir}");
    let Ok(files) = list_files(archive, suffix) else {
        return Vec::new();
    };

    files
        .into_iter()
        .filter(|path| path.starts_with(&dir_prefix))
        .filter_map(|path| {
            extract_file(archive, &path)
                .ok()
                .and_then(|content| parser(&content))
        })
        .collect()
}

fn discover_single_plugin(
    archive: &[u8],
    prefix: &str,
    plugin_path: &str,
) -> Result<PluginDescriptor> {
    let plugin_json_path = format!("{prefix}{plugin_path}/.claude-plugin/plugin.json");
    let alt_plugin_json_path = format!("{prefix}{plugin_path}/plugin.json");

    let plugin_content = extract_file(archive, &plugin_json_path)
        .or_else(|_| extract_file(archive, &alt_plugin_json_path))?;

    let plugin_json: PluginJson = serde_json::from_str(&plugin_content)?;

    let plugin_prefix = format!("{prefix}{plugin_path}/");

    let skills = scan_components(archive, &plugin_prefix, "skills/", "SKILL.md", |content| {
        parse_skill_descriptor(content).ok()
    });

    let commands = scan_components(archive, &plugin_prefix, "commands/", ".md", |content| {
        parse_command_descriptor(content, "command").ok()
    });

    let agents = scan_components(archive, &plugin_prefix, "agents/", ".md", |content| {
        parse_agent_descriptor(content).ok()
    });

    let hooks_path = format!("{prefix}{plugin_path}/.claude-plugin/hooks.json");
    let hooks = extract_file(archive, &hooks_path)
        .ok()
        .and_then(|content| parse_hooks_json(&content).ok());

    let mcp_path = format!("{prefix}{plugin_path}/.claude-plugin/.mcp.json");
    let mcp_servers = extract_file(archive, &mcp_path)
        .ok()
        .and_then(|content| parse_mcp_json(&content).ok())
        .unwrap_or_default();

    Ok(PluginDescriptor {
        name: plugin_json.name,
        description: plugin_json.description,
        skills,
        commands,
        agents,
        hooks,
        mcp_servers,
    })
}

pub fn discover_from_source(source: &PluginSource) -> Result<Vec<PluginDescriptor>> {
    match source {
        PluginSource::GitHub { github } => discover_plugins(github),
        PluginSource::Url { url } => discover_plugins(url),
        PluginSource::Relative(_) => Err(Error::NotFound(
            "Cannot discover from relative path without base URL".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_plugin_path_strips_prefix() {
        assert_eq!(resolve_plugin_path("./plugins/foo"), "plugins/foo");
        assert_eq!(resolve_plugin_path("plugins/bar"), "plugins/bar");
    }

    #[test]
    #[ignore = "requires network"]
    fn discover_anthropics_claude_code() {
        let plugins = discover_plugins("https://github.com/anthropics/claude-code").unwrap();
        assert!(
            plugins.len() >= 13,
            "Expected at least 13 plugins, got {}",
            plugins.len()
        );

        let names: Vec<_> = plugins.iter().map(|p| p.name.as_str()).collect();
        assert!(
            names.contains(&"Code Review"),
            "Should contain Code Review plugin"
        );
    }
}
