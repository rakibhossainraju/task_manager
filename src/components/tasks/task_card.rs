use dioxus::prelude::*;

use crate::models::Task;

#[component]
pub fn TaskCard(task_state: Task) -> Element {
    let title = &task_state.task.read().title;
    // let description = task_.description.read().as_deref().unwrap_or("").to_string();

    rsx! {
        div { class: "bg-white my-2 p-4 rounded shadow",
            h3 { "{title}" }
        }
    }
}
