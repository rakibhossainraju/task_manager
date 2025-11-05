mod data;
mod priority;

use dioxus::prelude::*;
use uuid::Uuid;

pub use crate::models::task::data::TaskData;

#[derive(Debug, Props, Clone, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub task: Signal<TaskData>,
}

impl Task {
    pub fn new(data: TaskData) -> Self {
        Self {
            id: data.id,
            task: Signal::new(data),
        }
    }
}
