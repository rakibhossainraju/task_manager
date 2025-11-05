use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::task::priority::TaskPriority;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TaskData {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl TaskData {
    pub fn new(
        id: Uuid,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
    ) -> Self {
        let mut task = Self::default();
        task.id = id;
        task.title = title;
        task.description = description;
        task.priority = priority;
        task
    }
}

impl TaskData {
    pub fn update_title(&mut self, new_title: impl Into<String>) {
        self.title = new_title.into();
        self.updated_at = Some(chrono::Local::now());
    }

    pub fn toggle_priority(&mut self) {
        self.priority = self.priority.toggle_priority();
        self.updated_at = Some(chrono::Local::now());
    }
    pub fn test(&self) {
        println!("Test function called on TaskData with id: {}", self.id);
    }
}
