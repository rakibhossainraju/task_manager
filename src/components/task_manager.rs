use dioxus::prelude::*;

use crate::{
    components::{Filter, TaskColumn},
    models::TasksGroupsState,
};

#[component]
pub fn TaskManager() -> Element {
    let task_groups = use_signal(TasksGroupsState::new);

    rsx! {
        div { class: "w-full h-screen bg-[#0E1116] flex flex-col font-sans select-none overflow-hidden",
            // Action & Filter Toolbar
            Filter {}

            // Kanban Columns Board Container
            main { class: "flex-1 px-6 pb-6 overflow-hidden w-full flex flex-col min-h-0",
                div { class: "flex items-stretch gap-[19px] w-full flex-1 overflow-x-auto min-h-0",
                    for group in task_groups.read().groups.read().iter() {
                        TaskColumn { task_group: *group, key: "{group.read().id}" }
                    }
                }
            }
        }
    }
}
