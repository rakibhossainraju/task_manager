use dioxus::prelude::*;

use crate::models::TasksGroupsState;

#[derive(Clone)]
pub struct TaskManagerContext {
    pub task_groups: Signal<TasksGroupsState>,
}

pub fn use_task_manager() -> TaskManagerContext {
    use_context()
}

