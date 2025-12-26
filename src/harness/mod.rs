//! Harness discovery and path resolution.

use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::types::{HarnessKind, Scope};

pub mod claude_code;

/// A discovered harness with resolved base paths.
///
/// Use [`Harness::locate`] to find a harness on the current system.
#[derive(Debug)]
pub struct Harness {
    kind: HarnessKind,
}

impl Harness {
    /// Locate a harness on the current system.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotFound`] if the harness is not installed.
    /// Returns [`Error::UnsupportedPlatform`] if the platform is not supported.
    ///
    /// [`Error::NotFound`]: crate::error::Error::NotFound
    /// [`Error::UnsupportedPlatform`]: crate::error::Error::UnsupportedPlatform
    pub fn locate(kind: HarnessKind) -> Result<Self> {
        match kind {
            HarnessKind::ClaudeCode => {
                if claude_code::is_installed() {
                    Ok(Self { kind })
                } else {
                    Err(Error::NotFound("Claude Code".into()))
                }
            }
            HarnessKind::OpenCode => Err(Error::NotFound("OpenCode not yet implemented".into())),
            HarnessKind::Goose => Err(Error::NotFound("Goose not yet implemented".into())),
        }
    }

    /// Returns the kind of harness.
    #[must_use]
    pub fn kind(&self) -> HarnessKind {
        self.kind
    }

    /// Returns the path to the skills directory for the given scope.
    #[must_use]
    pub fn skills_path(&self, scope: Scope) -> Option<PathBuf> {
        match self.kind {
            HarnessKind::ClaudeCode => claude_code::skills_dir(&scope),
            HarnessKind::OpenCode | HarnessKind::Goose => None,
        }
    }

    /// Returns the path to the commands directory for the given scope.
    #[must_use]
    pub fn commands_path(&self, scope: Scope) -> Option<PathBuf> {
        match self.kind {
            HarnessKind::ClaudeCode => claude_code::commands_dir(&scope).ok(),
            HarnessKind::OpenCode | HarnessKind::Goose => None,
        }
    }

    /// Returns the path to the config directory for the given scope.
    #[must_use]
    pub fn config_path(&self, scope: Scope) -> Option<PathBuf> {
        match self.kind {
            HarnessKind::ClaudeCode => claude_code::config_dir(&scope).ok(),
            HarnessKind::OpenCode | HarnessKind::Goose => None,
        }
    }

    /// Returns the path to the MCP configuration directory for the given scope.
    #[must_use]
    pub fn mcp_path(&self, scope: Scope) -> Option<PathBuf> {
        match self.kind {
            HarnessKind::ClaudeCode => claude_code::mcp_dir(&scope).ok(),
            HarnessKind::OpenCode | HarnessKind::Goose => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locate_claude_code_when_installed() {
        // This test only passes if Claude Code is installed
        if !claude_code::is_installed() {
            return;
        }

        let result = Harness::locate(HarnessKind::ClaudeCode);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), HarnessKind::ClaudeCode);
    }

    #[test]
    fn config_path_global_for_claude_code() {
        if !claude_code::is_installed() {
            return;
        }

        let harness = Harness::locate(HarnessKind::ClaudeCode).unwrap();
        let path = harness.config_path(Scope::Global);
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.is_absolute());
        assert!(path.ends_with(".claude"));
    }

    #[test]
    fn config_path_project_for_claude_code() {
        if !claude_code::is_installed() {
            return;
        }

        let harness = Harness::locate(HarnessKind::ClaudeCode).unwrap();
        let path = harness.config_path(Scope::Project(PathBuf::from("/some/project")));
        assert!(path.is_some());
        assert_eq!(path.unwrap(), PathBuf::from("/some/project/.claude"));
    }

    #[test]
    fn commands_path_for_claude_code() {
        if !claude_code::is_installed() {
            return;
        }

        let harness = Harness::locate(HarnessKind::ClaudeCode).unwrap();
        let path = harness.commands_path(Scope::Global);
        assert!(path.is_some());
        assert!(path.unwrap().ends_with("commands"));
    }

    #[test]
    fn skills_path_none_for_claude_code() {
        if !claude_code::is_installed() {
            return;
        }

        let harness = Harness::locate(HarnessKind::ClaudeCode).unwrap();
        assert!(harness.skills_path(Scope::Global).is_none());
    }
}
