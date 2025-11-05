use dioxus::prelude::*;

use crate::{components::TaskCard, models::TaskGroup};

#[component]
pub fn TaskColumn(task_group: TaskGroup) -> Element {
    let name = task_group.name;
    let description = task_group.description.as_deref().unwrap_or("");
    let task_list = task_group.task_list.read();

    rsx! {
        div { class: "bg-stone-100 my-2 p-4 rounded",
            div { class: "bg-stone-200 p-4 rounded",
                h3 { "{name}" }
                p { "{description}" }
            }
            for task_state in task_list.iter() {
                TaskCard { task_state: task_state.clone() }
            }
        }
    }
}
