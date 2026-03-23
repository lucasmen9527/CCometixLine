use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use crate::core::transcript::{TodoStatus, TranscriptAnalysis};
use std::collections::HashMap;

pub struct TodosSegment {
    transcript: TranscriptAnalysis,
}

impl TodosSegment {
    pub fn new(transcript: TranscriptAnalysis) -> Self {
        Self { transcript }
    }
}

impl Segment for TodosSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        if self.transcript.todos.is_empty() {
            return None;
        }

        let total = self.transcript.todos.len();
        let completed = self
            .transcript
            .todos
            .iter()
            .filter(|t| t.status == TodoStatus::Completed)
            .count();
        let in_progress = self
            .transcript
            .todos
            .iter()
            .find(|t| t.status == TodoStatus::InProgress);

        let primary = if let Some(current) = in_progress {
            let content = if current.content.len() > 30 {
                format!("{}...", &current.content[..27])
            } else {
                current.content.clone()
            };
            content
        } else if completed == total && total > 0 {
            "All done".to_string()
        } else {
            format!("{} pending", total - completed)
        };

        let secondary = format!("{}/{}", completed, total);

        let mut metadata = HashMap::new();
        metadata.insert("completed".to_string(), completed.to_string());
        metadata.insert("total".to_string(), total.to_string());
        metadata.insert(
            "in_progress".to_string(),
            in_progress.is_some().to_string(),
        );

        // Dynamic icon based on progress
        if completed == total && total > 0 {
            metadata.insert("dynamic_icon".to_string(), "\u{f0134}".to_string()); // nf-md-checkbox_marked
        } else if in_progress.is_some() {
            metadata.insert("dynamic_icon".to_string(), "\u{f0635}".to_string()); // nf-md-progress_check
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Todos
    }
}
