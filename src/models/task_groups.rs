use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Task;

pub type Tasks = Signal<Vec<Task>>;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TaskGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub task_list: Tasks,
}

pub struct TypeGroups {
    pub groups: Signal<Vec<TaskGroup>>,
}

impl TypeGroups {
    pub fn new() -> Self {
        Self {
            groups: Signal::new(vec![]),
        }
    }
}
