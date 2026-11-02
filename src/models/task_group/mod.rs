use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::task::Task;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "task_list", alias = "tasks")]
    pub tasks: Vec<Task>,
}

impl TaskGroup {
    #[allow(dead_code)]
    pub fn new(title: String, description: Option<String>, tasks: Option<Vec<Task>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            tasks: tasks.unwrap_or_default(),
        }
    }
}
