use dioxus::{html::dd, prelude::*};

use crate::models::{Task, TaskPriority};

#[component]
pub fn TaskCard(task: Task) -> Element {
    let title = task.title;
    let description = task.description.read().as_deref().unwrap_or("").to_string();

    rsx! {
        div { class: "bg-white my-2 p-4 rounded shadow",
            h3 { "{title}" }
            span {
                class: "font-bold text-sm border-gray-500 border-3 p-1 rounded",
                onclick: move |_| {
                    task.change_priority();
                },
                "{task.priority}"
            }
        }
    }
}