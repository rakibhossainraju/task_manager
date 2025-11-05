use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::TaskData;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGroupData {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub task_list: Vec<TaskData>,
}
