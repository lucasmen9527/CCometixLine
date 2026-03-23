use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use crate::core::transcript::{ToolStatus, TranscriptAnalysis};
use std::collections::HashMap;

pub struct ToolsSegment {
    transcript: TranscriptAnalysis,
}

impl ToolsSegment {
    pub fn new(transcript: TranscriptAnalysis) -> Self {
        Self { transcript }
    }
}

impl Segment for ToolsSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        if self.transcript.tools.is_empty() {
            return None;
        }

        let running: Vec<_> = self
            .transcript
            .tools
            .iter()
            .filter(|t| t.status == ToolStatus::Running)
            .collect();

        let completed_count = self
            .transcript
            .tools
            .iter()
            .filter(|t| t.status == ToolStatus::Completed)
            .count();

        let error_count = self
            .transcript
            .tools
            .iter()
            .filter(|t| t.status == ToolStatus::Error)
            .count();

        // Build primary text
        let mut parts = Vec::new();

        // Show running tools (up to 2)
        for tool in running.iter().take(2) {
            let target_str = tool
                .target
                .as_ref()
                .map(|t| format!(": {}", t))
                .unwrap_or_default();
            parts.push(format!("{}{}", tool.name, target_str));
        }
        if running.len() > 2 {
            parts.push(format!("+{}", running.len() - 2));
        }

        let primary = if !parts.is_empty() {
            parts.join(", ")
        } else {
            // No running tools, show top completed tools
            let mut tool_counts: HashMap<&str, usize> = HashMap::new();
            for tool in &self.transcript.tools {
                if tool.status == ToolStatus::Completed {
                    *tool_counts.entry(&tool.name).or_insert(0) += 1;
                }
            }
            let mut sorted: Vec<_> = tool_counts.into_iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
            sorted
                .iter()
                .take(3)
                .map(|(name, count)| format!("{}x{}", name, count))
                .collect::<Vec<_>>()
                .join(" ")
        };

        // Build secondary: summary counts
        let mut summary_parts = Vec::new();
        if !running.is_empty() {
            summary_parts.push(format!("{}act", running.len()));
        }
        if completed_count > 0 {
            summary_parts.push(format!("{}ok", completed_count));
        }
        if error_count > 0 {
            summary_parts.push(format!("{}err", error_count));
        }
        let secondary = if !summary_parts.is_empty() && parts.is_empty() {
            String::new() // Already showing counts in primary
        } else {
            summary_parts.join(" ")
        };

        let mut metadata = HashMap::new();
        metadata.insert("running".to_string(), running.len().to_string());
        metadata.insert("completed".to_string(), completed_count.to_string());
        metadata.insert("errors".to_string(), error_count.to_string());

        // Dynamic icon based on running state
        if !running.is_empty() {
            metadata.insert("dynamic_icon".to_string(), "\u{f1214}".to_string()); // nf-md-run
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Tools
    }
}
