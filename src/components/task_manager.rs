use dioxus::prelude::*;

use crate::{components::TaskColumn, models::TasksGroupsState};

#[component]
pub fn TaskManager() -> Element {
    let task_groups = use_signal(TasksGroupsState::new);

    let add_task_group = {
        let mut task_groups = task_groups.clone();
        move |_| {
            // task_groups.with_mut(|tg| {
            //     tg.add_task_group("Todo".to_string());
            // });
        }
    };
    rsx! {
        h1 { class: "font-extrabold text-2xl mb-4", "Task Manager" }
        button {
            class: "bg-blue-500 text-white px-4 py-2 rounded mb-4",
            onclick: add_task_group,
            "+ Add Task Group"
        }
        div {
            for group in task_groups.read().groups.iter() {
                TaskColumn { task_group: group.clone() }
            }
        }
    }
}
