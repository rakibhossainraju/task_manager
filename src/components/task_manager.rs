use dioxus::prelude::*;

use crate::{
    components::{Filter, TaskColumn},
    models::TasksGroupsState,
};

#[component]
pub fn TaskManager() -> Element {
    let task_groups = use_signal(TasksGroupsState::new);

    let add_task_group = {
        // move |_| {
        //     let mut task_groups = task_groups.clone();
        //     task_groups.with_mut(|tg| {
        //         tg.add_task_group("Test Group");
        //     });
        // }
    };
    info!("Re-Rendierng-The APP");
    rsx! {
        Filter {}
        // button {
        //     class: "bg-blue-500 text-white px-4 py-2 rounded mb-4",
        //     onclick: add_task_group,
        //     "+ Add Task Group"
        // }
        div {
            // for group in task_groups.read().groups.iter() {
            //     TaskColumn { task_group: group.clone() }
            // }
        }
    }
}
