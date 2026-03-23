use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use crate::core::transcript::{AgentStatus, TranscriptAnalysis};
use std::collections::HashMap;

pub struct AgentsSegment {
    transcript: TranscriptAnalysis,
}

impl AgentsSegment {
    pub fn new(transcript: TranscriptAnalysis) -> Self {
        Self { transcript }
    }
}

impl Segment for AgentsSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        if self.transcript.agents.is_empty() {
            return None;
        }

        let running: Vec<_> = self
            .transcript
            .agents
            .iter()
            .filter(|a| a.status == AgentStatus::Running)
            .collect();

        let completed_count = self
            .transcript
            .agents
            .iter()
            .filter(|a| a.status == AgentStatus::Completed)
            .count();

        // Build primary: show running agents info
        let primary = if !running.is_empty() {
            let agent_descs: Vec<String> = running
                .iter()
                .take(2)
                .map(|a| {
                    let desc = a
                        .description
                        .as_ref()
                        .map(|d| {
                            if d.len() > 20 {
                                format!(": {}...", &d[..17])
                            } else {
                                format!(": {}", d)
                            }
                        })
                        .unwrap_or_default();
                    format!("{}{}", a.agent_type, desc)
                })
                .collect();
            if running.len() > 2 {
                format!("{} +{}", agent_descs.join(", "), running.len() - 2)
            } else {
                agent_descs.join(", ")
            }
        } else {
            // All completed
            format!("{} done", completed_count)
        };

        let secondary = if !running.is_empty() && completed_count > 0 {
            format!("{}ok", completed_count)
        } else {
            String::new()
        };

        let mut metadata = HashMap::new();
        metadata.insert("running".to_string(), running.len().to_string());
        metadata.insert("completed".to_string(), completed_count.to_string());

        if !running.is_empty() {
            metadata.insert("dynamic_icon".to_string(), "\u{f0219}".to_string()); // nf-md-robot_outline (active)
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Agents
    }
}
