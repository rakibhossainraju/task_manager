mod data;
mod state;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{Task, task_group::data::TaskGroupData};
pub use state::TasksGroupsState;

type TaskGroupsData = Vec<TaskGroupData>;

pub type Tasks = Signal<Vec<Task>>;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TaskGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub task_list: Tasks,
}

impl From<TaskGroupData> for TaskGroup {
    fn from(data: TaskGroupData) -> Self {
        Self::new(data)
    }
}

impl TaskGroup {
    fn new(data: TaskGroupData) -> Self {
        let task_list: Vec<Task> = data.task_list.into_iter().map(Task::from).collect();

        Self {
            id: data.id,
            name: data.name,
            description: data.description,
            task_list: Signal::new(task_list),
        }
    }
}
