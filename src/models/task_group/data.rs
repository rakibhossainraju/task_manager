use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::TaskData;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGroupData {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub task_list: Vec<TaskData>,
}

impl TaskGroupData {
    pub fn new(title: String, description: Option<String>, task_list: Option<Vec<TaskData>>) -> Self {
        let description = description.map(|d| d.into());
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            task_list: task_list.unwrap_or(Vec::new()),
        }
    }
}