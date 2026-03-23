use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ToolEntry {
    pub id: String,
    pub name: String,
    pub target: Option<String>,
    pub status: ToolStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolStatus {
    Running,
    Completed,
    Error,
}

#[derive(Debug, Clone)]
pub struct AgentEntry {
    pub id: String,
    pub agent_type: String,
    pub model: Option<String>,
    pub description: Option<String>,
    pub status: AgentStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentStatus {
    Running,
    Completed,
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub content: String,
    pub status: TodoStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Default)]
pub struct TranscriptAnalysis {
    pub tools: Vec<ToolEntry>,
    pub agents: Vec<AgentEntry>,
    pub todos: Vec<TodoItem>,
}

#[derive(Deserialize)]
struct TranscriptLine {
    #[serde(default)]
    message: Option<TranscriptMessage>,
}

#[derive(Deserialize)]
struct TranscriptMessage {
    #[serde(default)]
    content: Option<Vec<ContentBlock>>,
}

#[derive(Deserialize)]
struct ContentBlock {
    r#type: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    input: Option<serde_json::Value>,
    #[serde(default)]
    tool_use_id: Option<String>,
    #[serde(default)]
    is_error: Option<bool>,
}

impl TranscriptAnalysis {
    pub fn parse<P: AsRef<Path>>(transcript_path: P) -> Self {
        let path = transcript_path.as_ref();
        if !path.exists() {
            return Self::default();
        }

        let file = match fs::File::open(path) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };

        let reader = BufReader::new(file);
        let mut tool_map: HashMap<String, ToolEntry> = HashMap::new();
        let mut agent_map: HashMap<String, AgentEntry> = HashMap::new();
        let mut todos: Vec<TodoItem> = Vec::new();
        let mut task_id_to_index: HashMap<String, usize> = HashMap::new();

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }

            let entry: TranscriptLine = match serde_json::from_str(&line) {
                Ok(e) => e,
                Err(_) => continue,
            };

            let content = match entry.message.and_then(|m| m.content) {
                Some(c) => c,
                None => continue,
            };

            for block in content {
                match block.r#type.as_str() {
                    "tool_use" => {
                        let id = match &block.id {
                            Some(id) => id.clone(),
                            None => continue,
                        };
                        let name = match &block.name {
                            Some(name) => name.clone(),
                            None => continue,
                        };

                        if name == "Task" || name == "Agent" {
                            // Agent entry
                            let input = block.input.as_ref();
                            let agent_type = input
                                .and_then(|v| v.get("subagent_type"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            let model = input
                                .and_then(|v| v.get("model"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());
                            let description = input
                                .and_then(|v| v.get("description"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            agent_map.insert(
                                id,
                                AgentEntry {
                                    id: block.id.unwrap_or_default(),
                                    agent_type,
                                    model,
                                    description,
                                    status: AgentStatus::Running,
                                },
                            );
                        } else if name == "TodoWrite" {
                            // Replace all todos
                            if let Some(input) = &block.input {
                                if let Some(todo_arr) = input.get("todos").and_then(|v| v.as_array())
                                {
                                    todos.clear();
                                    task_id_to_index.clear();
                                    for item in todo_arr {
                                        let content = item
                                            .get("content")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        let status = parse_todo_status(
                                            item.get("status").and_then(|v| v.as_str()),
                                        );
                                        todos.push(TodoItem { content, status });
                                    }
                                }
                            }
                        } else if name == "TaskCreate" {
                            if let Some(input) = &block.input {
                                let subject = input
                                    .get("subject")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                let description = input
                                    .get("description")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                let content = if !subject.is_empty() {
                                    subject.to_string()
                                } else if !description.is_empty() {
                                    description.to_string()
                                } else {
                                    "Untitled task".to_string()
                                };
                                let status = parse_todo_status(
                                    input.get("status").and_then(|v| v.as_str()),
                                );
                                todos.push(TodoItem { content, status });

                                let task_id = input
                                    .get("taskId")
                                    .and_then(|v| {
                                        v.as_str()
                                            .map(|s| s.to_string())
                                            .or_else(|| v.as_u64().map(|n| n.to_string()))
                                    })
                                    .unwrap_or_else(|| id.clone());
                                task_id_to_index.insert(task_id, todos.len() - 1);
                            }
                        } else if name == "TaskUpdate" {
                            if let Some(input) = &block.input {
                                let index =
                                    resolve_task_index(input, &task_id_to_index, todos.len());
                                if let Some(idx) = index {
                                    if let Some(status_str) =
                                        input.get("status").and_then(|v| v.as_str())
                                    {
                                        let status = parse_todo_status(Some(status_str));
                                        todos[idx].status = status;
                                    }
                                    let subject = input
                                        .get("subject")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    let desc = input
                                        .get("description")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    let content = if !subject.is_empty() {
                                        subject
                                    } else if !desc.is_empty() {
                                        desc
                                    } else {
                                        ""
                                    };
                                    if !content.is_empty() {
                                        todos[idx].content = content.to_string();
                                    }
                                }
                            }
                        } else {
                            // Regular tool
                            let target = extract_tool_target(&name, block.input.as_ref());
                            tool_map.insert(
                                id,
                                ToolEntry {
                                    id: block.id.unwrap_or_default(),
                                    name,
                                    target,
                                    status: ToolStatus::Running,
                                },
                            );
                        }
                    }
                    "tool_result" => {
                        if let Some(tool_use_id) = &block.tool_use_id {
                            let is_error = block.is_error.unwrap_or(false);
                            if let Some(tool) = tool_map.get_mut(tool_use_id) {
                                tool.status = if is_error {
                                    ToolStatus::Error
                                } else {
                                    ToolStatus::Completed
                                };
                            }
                            if let Some(agent) = agent_map.get_mut(tool_use_id) {
                                agent.status = AgentStatus::Completed;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Keep last 20 tools, last 10 agents
        let tools: Vec<ToolEntry> = tool_map
            .into_values()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        let agents: Vec<AgentEntry> = agent_map
            .into_values()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .take(10)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        TranscriptAnalysis {
            tools,
            agents,
            todos,
        }
    }
}

fn extract_tool_target(tool_name: &str, input: Option<&serde_json::Value>) -> Option<String> {
    let input = input?;
    match tool_name {
        "Read" | "Write" | "Edit" => input
            .get("file_path")
            .or_else(|| input.get("path"))
            .and_then(|v| v.as_str())
            .map(|s| truncate_path(s, 20)),
        "Glob" | "Grep" => input
            .get("pattern")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        "Bash" => input.get("command").and_then(|v| v.as_str()).map(|s| {
            if s.len() > 25 {
                format!("{}...", &s[..25])
            } else {
                s.to_string()
            }
        }),
        _ => None,
    }
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        return path.to_string();
    }
    let parts: Vec<&str> = path.split('/').collect();
    let filename = parts.last().unwrap_or(&path);
    if filename.len() >= max_len {
        return format!("{}...", &filename[..max_len.saturating_sub(3)]);
    }
    format!(".../{}", filename)
}

fn parse_todo_status(status: Option<&str>) -> TodoStatus {
    match status {
        Some("in_progress") | Some("running") => TodoStatus::InProgress,
        Some("completed") | Some("complete") | Some("done") => TodoStatus::Completed,
        _ => TodoStatus::Pending,
    }
}

fn resolve_task_index(
    input: &serde_json::Value,
    task_id_to_index: &HashMap<String, usize>,
    todos_len: usize,
) -> Option<usize> {
    let task_id = input.get("taskId")?;
    let key = if let Some(s) = task_id.as_str() {
        s.to_string()
    } else if let Some(n) = task_id.as_u64() {
        n.to_string()
    } else {
        return None;
    };

    if let Some(&idx) = task_id_to_index.get(&key) {
        return Some(idx);
    }

    // Try numeric index (1-based)
    if let Ok(num) = key.parse::<usize>() {
        let idx = num.saturating_sub(1);
        if idx < todos_len {
            return Some(idx);
        }
    }

    None
}
