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
impl TaskGroup {
    fn new(data: TaskGroupData) -> Self {
        let task_list: Tasks = Signal::new(data.task_list.into_iter().map(Task::new).collect());

        Self {
            id: data.id,
            name: data.name,
            description: data.description,
            task_list,
        }
    }
}

pub struct TasksGroupsState {
    pub groups: Signal<Vec<TaskGroup>>,
}

impl TasksGroupsState {
    pub fn new() -> Self {
        Self::load_groups();

        Self {
            groups: Self::load_groups(),
        }
    }
    pub fn load_groups() -> Signal<Vec<TaskGroup>> {
        let json_str = include_str!("./task_groups.json");
        let groups_data: TasksGroupsData =
            serde_json::from_str(json_str).expect("Failed to parse task_groups.json");

        let mut groups: Vec<TaskGroup> = Vec::new();
        for group_data in groups_data {
            let group = TaskGroup::new(group_data);
            groups.push(group);
        }
        Signal::new(groups)
    }
}
