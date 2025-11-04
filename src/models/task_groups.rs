// use dioxus::signals::Signal;

use dioxus::prelude::*;

use crate::models::Task;

pub type Tasks = Signal<Vec<Task>>;

#[derive(Debug, Clone, PartialEq, Props)]
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
        // let json_str = include_str!("./task_groups.json");
        // let groups: TaskGroupsType = serde_json::from_str(json_str)
        //     .expect("Failed to parse task_groups.json");

        Self { groups: vec![] }
    }
    pub fn add_task_group(&mut self, name: String) {
        let mut task_list: Tasks = Signal::new(vec![]);
        task_list.write().push(
            Task {
            id: 2324 + self.groups.len() as u128,
            title: Signal::new("New Task 1".to_string()),
            description: Signal::new(None),
            priority: Signal::new(crate::models::TaskPriority::Low),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: None,
        }
        );
        task_list.write().push(
            Task {
            id: 23234 + self.groups.len() as u128,
            title: Signal::new("New Task 2".to_string()),
            description: Signal::new(None),
            priority: Signal::new(crate::models::TaskPriority::Low),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: None,
        }
        );



        let new_group = TaskGroup {
            id: 23243 + self.groups.len() as u128,
            name,
            description: None,
            task_list,
        };
        self.groups.push(new_group);
    }
}

