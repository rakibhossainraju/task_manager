use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Task, TaskData};

#[derive(Debug, Serialize, Deserialize)]
struct TaskGroupData {
    id: Uuid,
    name: String,
    description: Option<String>,
    task_list: Vec<TaskData>,
}

type TasksGroupsData = Vec<TaskGroupData>;

pub type Tasks = Signal<Vec<Task>>;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TaskGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub task_list: Tasks,
}

pub struct TasksGroupsState {
    pub groups: Signal<Vec<TaskGroup>>,
}

impl TasksGroupsState {
    pub fn new() -> Self {
        Self::load_groups();

        Self {
            groups: Signal::new(vec![]),
        }
    }
    pub fn load_groups() {
        let json_str = include_str!("./task_groups.json");
        let groups: TasksGroupsData =
            serde_json::from_str(json_str).expect("Failed to parse task_groups.json");
        info!("Loaded task groups: {:#?}", groups);
    }
}
