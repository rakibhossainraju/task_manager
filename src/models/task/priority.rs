use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    #[serde(alias = "Mid")]
    Medium,
    High,
    Urgent,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Low
    }
}

impl Display for TaskPriority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let priority_str = match self {
            TaskPriority::Low => "Low",
            TaskPriority::Medium => "Mid",
            TaskPriority::High => "High",
            TaskPriority::Urgent => "Urgent",
        };
        write!(f, "{}", priority_str)
    }
}

impl TaskPriority {
    pub fn colors(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            TaskPriority::Urgent => ("#090441", "#4F39F6", "#4F39F6"),
            TaskPriority::High => ("#2D0506", "#E7000B", "#E7000B"),
            TaskPriority::Medium => ("#2C2300", "#FFD230", "#FFD230"),
            TaskPriority::Low => ("#101828", "#8798B0", "#8798B0"),
        }
    }

    #[allow(dead_code)]
    pub fn toggle_priority(&self) -> TaskPriority {
        match self {
            TaskPriority::Low => TaskPriority::Medium,
            TaskPriority::Medium => TaskPriority::High,
            TaskPriority::High => TaskPriority::Urgent,
            TaskPriority::Urgent => TaskPriority::Low,
        }
    }
}
