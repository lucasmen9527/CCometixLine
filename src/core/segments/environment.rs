use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct EnvironmentSegment;

impl EnvironmentSegment {
    pub fn new() -> Self {
        Self
    }

    fn count_claude_md(cwd: &str) -> u32 {
        let mut count = 0u32;

        // ~/.claude/CLAUDE.md
        if let Some(home) = dirs::home_dir() {
            let user_claude_md = home.join(".claude").join("CLAUDE.md");
            if user_claude_md.exists() {
                count += 1;
            }
        }

        // {cwd}/CLAUDE.md
        let project_claude_md = Path::new(cwd).join("CLAUDE.md");
        if project_claude_md.exists() {
            count += 1;
        }

        // {cwd}/CLAUDE.local.md
        let project_local = Path::new(cwd).join("CLAUDE.local.md");
        if project_local.exists() {
            count += 1;
        }

        // {cwd}/.claude/CLAUDE.md
        let nested = Path::new(cwd).join(".claude").join("CLAUDE.md");
        if nested.exists() {
            count += 1;
        }

        count
    }

    fn count_rules(cwd: &str) -> u32 {
        let mut count = 0u32;

        // ~/.claude/rules/
        if let Some(home) = dirs::home_dir() {
            count += count_md_files_recursive(&home.join(".claude").join("rules"));
        }

        // {cwd}/.claude/rules/
        count += count_md_files_recursive(&Path::new(cwd).join(".claude").join("rules"));

        count
    }

    fn count_mcps(cwd: &str) -> u32 {
        let mut count = 0u32;

        // ~/.claude/settings.json → mcpServers
        if let Some(home) = dirs::home_dir() {
            count += count_json_object_keys(
                &home.join(".claude").join("settings.json"),
                "mcpServers",
            );
        }

        // {cwd}/.mcp.json → mcpServers
        count += count_json_object_keys(&Path::new(cwd).join(".mcp.json"), "mcpServers");

        // {cwd}/.claude/settings.json → mcpServers
        count += count_json_object_keys(
            &Path::new(cwd).join(".claude").join("settings.json"),
            "mcpServers",
        );

        count
    }

    fn count_hooks(cwd: &str) -> u32 {
        let mut count = 0u32;

        // ~/.claude/settings.json → hooks
        if let Some(home) = dirs::home_dir() {
            count +=
                count_json_object_keys(&home.join(".claude").join("settings.json"), "hooks");
        }

        // {cwd}/.claude/settings.json → hooks
        count += count_json_object_keys(
            &Path::new(cwd).join(".claude").join("settings.json"),
            "hooks",
        );

        count
    }
}

impl Segment for EnvironmentSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let cwd = &input.workspace.current_dir;

        let claude_md = Self::count_claude_md(cwd);
        let rules = Self::count_rules(cwd);
        let mcps = Self::count_mcps(cwd);
        let hooks = Self::count_hooks(cwd);

        let total = claude_md + rules + mcps + hooks;
        if total == 0 {
            return None;
        }

        let mut parts = Vec::new();
        if claude_md > 0 {
            parts.push(format!("{} CLAUDE.md", claude_md));
        }
        if rules > 0 {
            parts.push(format!("{} rules", rules));
        }
        if mcps > 0 {
            parts.push(format!("{} MCPs", mcps));
        }
        if hooks > 0 {
            parts.push(format!("{} hooks", hooks));
        }

        let primary = parts.join(" · ");

        let mut metadata = HashMap::new();
        metadata.insert("claude_md".to_string(), claude_md.to_string());
        metadata.insert("rules".to_string(), rules.to_string());
        metadata.insert("mcps".to_string(), mcps.to_string());
        metadata.insert("hooks".to_string(), hooks.to_string());

        Some(SegmentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Environment
    }
}

fn count_md_files_recursive(dir: &Path) -> u32 {
    if !dir.exists() {
        return 0;
    }
    let mut count = 0u32;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                count += count_md_files_recursive(&path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                count += 1;
            }
        }
    }
    count
}

fn count_json_object_keys(file_path: &Path, key: &str) -> u32 {
    if !file_path.exists() {
        return 0;
    }
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return 0,
    };
    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    match json.get(key) {
        Some(serde_json::Value::Object(obj)) => obj.len() as u32,
        _ => 0,
    }
}
