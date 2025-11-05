use dioxus::prelude::*;

use crate::models::{TaskGroup, task_group::TaskGroupsData};

pub struct TasksGroupsState {
    pub groups: Signal<Vec<TaskGroup>>,
}

impl TasksGroupsState {
    pub fn new() -> Self {
        Self {
            groups: Self::load_groups(),
        }
    }
    pub fn load_groups() -> Signal<Vec<TaskGroup>> {
        let json_str = include_str!("./task_groups.json");
        let groups_data: TaskGroupsData =
            serde_json::from_str(json_str).expect("Failed to parse task_groups.json");

        let mut groups: Vec<TaskGroup> = Vec::new();
        for group_data in groups_data {
            let group = TaskGroup::new(group_data);
            groups.push(group);
        }
        Signal::new(groups)
    }
}
