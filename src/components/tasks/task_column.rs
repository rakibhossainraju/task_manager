use dioxus::prelude::*;

use crate::{components::TaskCard, models::TaskGroup};

#[component]
pub fn TaskColumn(task_group: Signal<TaskGroup>) -> Element {
    let task_group_read = task_group.read();
    let title = task_group_read.title.as_str();
    let description = task_group_read.description.as_deref().unwrap_or("");
    let change_group_title = {
        let mut task_group = task_group.clone();
        move |_| {
            task_group.write().update_group_title("New Title!");
        }
    };
    info!("Re-Rendierng-column");

    rsx! {
        div { class: "bg-stone-100 my-2 p-4 rounded",
            div { class: "bg-stone-200 p-4 rounded",
                h3 {class: "font-medium text-lg", onclick: change_group_title, "{title}" }
                p { "{description}" }
            }
            for task in task_group_read.task_list.iter() {
                TaskCard { task_signal: task.task, key: "{task.id}" }
            }
        }
    }
}

