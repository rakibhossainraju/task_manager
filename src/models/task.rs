use std::fmt::{Display, Formatter};

use chrono::Local;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
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
            TaskPriority::Medium => "Medium",
            TaskPriority::High => "High",
            TaskPriority::Urgent => "Urgent",
        };
        write!(f, "{}", priority_str)
    }
}
impl TaskPriority {
    pub fn match_and_get_next(&self) -> TaskPriority {
        match self {
            TaskPriority::Low => TaskPriority::Medium,
            TaskPriority::Medium => TaskPriority::High,
            TaskPriority::High => TaskPriority::Urgent,
            TaskPriority::Urgent => TaskPriority::Low,
        }
    }
}

#[derive(Debug, Props, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskData {
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl Default for TaskData {
    fn default() -> Self {
        Self {
            title: "Untitled Task".into(),
            description: None,
            priority: TaskPriority::default(),
            created_at: Local::now().to_string(),
            updated_at: None,
        }
    }
}

#[derive(Debug, Props, Clone, PartialEq)]
pub struct Task {
    pub id: u128,
    pub task: Signal<TaskData>,
}

impl Task {
    pub fn new(id: u128, title: String) -> Self {}
    pub fn change_priority(&mut self) {
        let next_priority = (self.priority)().match_and_get_next();
        self.priority.set(next_priority);
    }
}
