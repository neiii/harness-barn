//! Claude Code harness implementation.
//!
//! Claude Code stores its configuration in:
//! - **Global**: `$CLAUDE_CONFIG_DIR` or `~/.claude/`
//! - **Project**: `.claude/` in project root

use std::path::PathBuf;

use crate::error::Result;
use crate::platform;
use crate::types::Scope;

/// Environment variable for Claude Code config directory override.
const CLAUDE_CONFIG_DIR_ENV: &str = "CLAUDE_CONFIG_DIR";

/// Returns the global Claude Code configuration directory.
///
/// Checks `CLAUDE_CONFIG_DIR` environment variable first, then falls back
/// to `~/.claude/`.
///
/// # Errors
///
/// Returns an error if the home directory cannot be determined and
/// no environment variable is set.
pub fn global_config_dir() -> Result<PathBuf> {
    // Check environment variable first
    if let Ok(dir) = std::env::var(CLAUDE_CONFIG_DIR_ENV) {
        let path = PathBuf::from(dir);
        if path.is_absolute() {
            return Ok(path);
        }
    }

    // Fall back to ~/.claude/
    Ok(platform::home_dir()?.join(".claude"))
}

/// Returns the project-local Claude Code configuration directory.
///
/// # Arguments
///
/// * `project_root` - Path to the project root directory
#[must_use]
pub fn project_config_dir(project_root: &std::path::Path) -> PathBuf {
    project_root.join(".claude")
}

/// Returns the commands directory for the given scope.
///
/// - **Global**: `~/.claude/commands/`
/// - **Project**: `.claude/commands/`
pub fn commands_dir(scope: &Scope) -> Result<PathBuf> {
    match scope {
        Scope::Global => Ok(global_config_dir()?.join("commands")),
        Scope::Project(root) => Ok(project_config_dir(root).join("commands")),
    }
}

/// Returns the config directory for the given scope.
///
/// This is the base configuration directory.
pub fn config_dir(scope: &Scope) -> Result<PathBuf> {
    match scope {
        Scope::Global => global_config_dir(),
        Scope::Project(root) => Ok(project_config_dir(root)),
    }
}

/// Returns the MCP configuration directory for the given scope.
///
/// Claude Code stores MCP configuration in the base config directory
/// (settings files like `.mcp.json`).
pub fn mcp_dir(scope: &Scope) -> Result<PathBuf> {
    config_dir(scope)
}

/// Returns the skills directory for the given scope.
///
/// Claude Code does not have a dedicated skills directory, so this
/// returns `None` for both global and project scopes.
#[must_use]
pub fn skills_dir(_scope: &Scope) -> Option<PathBuf> {
    None
}

/// Checks if Claude Code is installed on this system.
///
/// Currently checks if the global config directory exists.
pub fn is_installed() -> bool {
    global_config_dir().map(|p| p.exists()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_config_dir_is_absolute() {
        // Skip if home dir cannot be determined (CI environments)
        if platform::home_dir().is_err() {
            return;
        }

        let result = global_config_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.is_absolute());
        assert!(path.ends_with(".claude"));
    }

    #[test]
    fn project_config_dir_is_relative_to_root() {
        let root = PathBuf::from("/some/project");
        let config = project_config_dir(&root);
        assert_eq!(config, PathBuf::from("/some/project/.claude"));
    }

    #[test]
    fn commands_dir_global() {
        if platform::home_dir().is_err() {
            return;
        }

        let result = commands_dir(&Scope::Global);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("commands"));
    }

    #[test]
    fn commands_dir_project() {
        let root = PathBuf::from("/some/project");
        let result = commands_dir(&Scope::Project(root));
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, PathBuf::from("/some/project/.claude/commands"));
    }

    #[test]
    fn skills_dir_returns_none() {
        assert!(skills_dir(&Scope::Global).is_none());
        assert!(skills_dir(&Scope::Project(PathBuf::from("/project"))).is_none());
    }
}
