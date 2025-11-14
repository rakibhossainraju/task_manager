mod data;
mod priority;

use chrono::{DateTime, Local};
use uuid::Uuid;

pub use crate::models::task::data::TaskData;
use crate::models::task::priority::TaskPriority;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl From<TaskData> for Task {
    fn from(data: TaskData) -> Self {
        Self::new(data)
    }
}

impl Task {
    pub fn new(data: TaskData) -> Self {
        Self {
            id: data.id,
            title: data.title,
            description: data.description,
            priority: data.priority,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }
    pub fn update_title(&mut self, new_title: impl Into<String>) {
        self.title = new_title.into();
        self.updated_at = Some(Local::now());
    }

    pub fn toggle_priority(&mut self) {
        self.priority = self.priority.toggle_priority();
        self.updated_at = Some(Local::now());
    }
}
