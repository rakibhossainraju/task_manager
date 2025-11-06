mod data;
mod state;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{Task, task_group::data::TaskGroupData};
pub use state::TasksGroupsState;

type TaskGroupsData = Vec<TaskGroupData>;

pub type TasksType = Signal<Vec<Signal<Task>>>;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TaskGroup {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub task_list: TasksType,
}

impl From<TaskGroupData> for TaskGroup {
    fn from(data: TaskGroupData) -> Self {
        Self::new(data)
    }
}

impl TaskGroup {
    fn new(data: TaskGroupData) -> Self {
        let task_list: Vec<Signal<Task>> = data.task_list.into_iter().map(|t| Signal::new(Task::from(t))).collect();

        Self {
            id: data.id,
            title: data.title,
            description: data.description,
            task_list: Signal::new(task_list),
        }
    }
    pub fn update_group_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }
}
