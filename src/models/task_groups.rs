// use dioxus::signals::Signal;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::Task;

pub type Tasks = Vec<Task>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Props)]
pub struct TaskGroup {
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
    pub task_list: Tasks,
}

pub type TaskGroupsType = Vec<TaskGroup>;

pub struct TaskGroups {
    pub groups: TaskGroupsType,
}
impl TaskGroups {
    pub fn new() -> Self {
        let json_str = include_str!("./task_groups.json");
        let groups: TaskGroupsType = serde_json::from_str(json_str)
            .expect("Failed to parse task_groups.json");

        Self { groups }
    }
}

