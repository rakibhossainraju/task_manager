use dioxus::prelude::*;

use crate::{components::TaskColumn, models::TaskGroups};


#[component]
pub fn TaskManager() -> Element {
    let task_groups = use_signal(TaskGroups::new);
    rsx! {
        h1 { class: "font-extrabold text-2xl mb-4", "Task Manager" }
        div {
            for group in task_groups.read().groups.iter() {
                TaskColumn { task_group: group.clone() }
            }
        }
    }
}
