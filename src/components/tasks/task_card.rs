use dioxus::prelude::*;

use crate::models::Task;

#[component]
pub fn TaskCard(task: Task) -> Element {
    let title = task.title;
    let description = task.description.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "bg-white my-2 p-4 rounded shadow",
            h3 { "{title}" }
            p { "{description}" }
        
        }
    }
}