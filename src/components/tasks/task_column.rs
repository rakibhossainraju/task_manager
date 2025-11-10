use dioxus::prelude::*;

use crate::{components::TaskCard, models::TaskGroup};

#[component]
pub fn TaskColumn(task_group: Signal<TaskGroup>) -> Element {
    let group = task_group.read();

    let title = &group.title;
    let description = group.description.as_deref().unwrap_or("");

    let change_group_title = move |_| {
        task_group.write().update_group_title("New Title!");
    };

    info!("Re-Rendierng-column");

    rsx! {
        div { class: "bg-stone-100 my-2 p-4 rounded",
            div { class: "bg-stone-200 p-4 rounded",
                h3 { class: "font-medium text-lg", onclick: change_group_title, "{title}" }
                p { "{description}" }
            }
            for task_signal in group.task_list.iter() {
                TaskCard { task_signal: *task_signal, key: "{task_signal.read().id}" }
            }
        }
    }
}
