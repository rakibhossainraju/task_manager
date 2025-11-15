use dioxus::prelude::*;

use crate::{
    components::{Filter, TaskColumn},
    models::use_task_manager,
};

#[component]
pub fn TaskManager() -> Element {
    let ctx = use_task_manager();
    let task_groups = ctx.task_groups;

    rsx! {
        Filter {}
        div { class: "flex gap-4 mt-8 pb-2 overflow-x-auto",
            for group in task_groups.read().iter() {
                TaskColumn { group: group.clone() }
            }
        }
    }
}
