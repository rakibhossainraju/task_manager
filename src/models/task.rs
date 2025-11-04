use std::fmt::{Display, Formatter};

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
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


#[derive(Debug, Props, Clone, PartialEq)]
pub struct Task {
    pub id: u128,
    pub title: Signal<String>,
    pub description: Signal<Option<String>>,
    pub priority: Signal<TaskPriority>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl Task {
    pub fn new(id: u128, title: String) -> Self {
        Self {
            id,
            title: Signal::new(title),
            description: Signal::new(None),
            priority: Signal::new(TaskPriority::default()),
            created_at: "2024-06-06T14:00:00Z".to_string(),
            updated_at: None,
        }
    }
    pub fn change_priority(&mut self) {
        let next_priority = (self.priority)().match_and_get_next();
        self.priority.set(next_priority);
    }
}