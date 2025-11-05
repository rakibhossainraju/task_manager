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
