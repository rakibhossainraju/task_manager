use dioxus::prelude::*;

use crate::models::{TaskGroup, task_group::TaskGroupsData};
use crate::models::task_group::data::TaskGroupData;

type GroupsType = Signal<Vec<Signal<TaskGroup>>>;
pub struct TasksGroupsState {
    pub groups: GroupsType,
}

impl TasksGroupsState {
    pub fn new() -> Self {
        Self {
            groups: Self::load_groups(),
        }
    }
    pub fn load_groups() -> GroupsType {
        let json_str = include_str!("./task_groups.json");
        let groups_data: TaskGroupsData =
            serde_json::from_str(json_str).expect("Failed to parse task_groups.json");

        let groups: Vec<Signal<TaskGroup>> = groups_data.into_iter().map(|g| Signal::new(TaskGroup::from(g))).collect();
        Signal::new(groups)
    }
    pub fn add_task_group(&mut self, group_name: impl Into<String>) {
        let group_name = group_name.into();
        let group: TaskGroup = TaskGroupData::new(group_name, Some("This is description".to_string()), None).into();
        self.groups.write().push(Signal::new(group));
    }
}
