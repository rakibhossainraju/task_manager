pub mod priority;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use priority::TaskPriority;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(title: String, description: Option<String>, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            priority,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}
