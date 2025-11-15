mod context;
mod data;
mod state;

use uuid::Uuid;

use crate::models::Task;
pub use context::{TaskManagerContext, use_task_manager};
pub use state::{TasksGroupsState, add_task_group, load_task_groups, update_group, update_task};

pub(crate) type TaskGroupsData = Vec<data::TaskGroupData>;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskGroup {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub task_list: Vec<Task>,
}

impl From<data::TaskGroupData> for TaskGroup {
    fn from(data: data::TaskGroupData) -> Self {
        Self::new(data)
    }
}

impl TaskGroup {
    fn new(data: data::TaskGroupData) -> Self {
        let task_list: Vec<Task> = data.task_list.into_iter().map(Task::from).collect();

        Self {
            id: data.id,
            title: data.title,
            description: data.description,
            task_list,
        }
    }

    pub fn update_group_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn add_task(&mut self, task: Task) {
        self.task_list.push(task);
    }

    pub fn update_task(&mut self, task_id: Uuid, f: impl FnOnce(&mut Task)) {
        if let Some(task) = self.task_list.iter_mut().find(|t| t.id == task_id) {
            f(task);
        }
    }
}
