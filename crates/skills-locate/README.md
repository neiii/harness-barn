# skills-locate

Skill discovery and fetching for AI coding agents.

## Features

- Parse GitHub references (`github:owner/repo`, `github:owner/repo@ref`)
- Fetch and extract files from GitHub archives
- Parse plugin manifests (`plugin.json`, `marketplace.json`)
- Parse skill descriptors (agents, commands, skills)
- Parse hooks and MCP server configs

## Usage

```rust
use skills_locate::{GitHubRef, discover_plugins, PluginDescriptor};

// Parse a GitHub reference
let source = GitHubRef::parse("github:anthropics/claude-code-plugins")?;

// Discover all plugins from source
let plugins: Vec<PluginDescriptor> = discover_plugins(&source)?;

for plugin in plugins {
    println!("{}: {}", plugin.name, plugin.description);
    
    for skill in &plugin.skills {
        println!("  skill: {}", skill.name);
    }
}
```

## Types

| Type | Purpose |
|------|---------|
| `GitHubRef` | Parsed GitHub URL (owner, repo, ref) |
| `PluginDescriptor` | Full plugin metadata with skills, commands, agents |
| `SkillDescriptor` | Individual skill definition |
| `CommandDescriptor` | Slash command definition |
| `AgentDescriptor` | Agent/subagent definition |
| `Marketplace` | Registry of available plugins |

## Parsing Components

```rust
use skills_locate::{parse_skill_descriptor, parse_command_descriptor, parse_agent_descriptor};

// Parse markdown skill files
let skill = parse_skill_descriptor("skill.md", content)?;

// Parse YAML frontmatter commands
let command = parse_command_descriptor("command.md", content)?;

// Parse agent definitions
let agent = parse_agent_descriptor("agent.md", content)?;
```

## License

MIT
