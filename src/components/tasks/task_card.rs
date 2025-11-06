use dioxus::prelude::*;

use crate::models::Task;

#[component]
pub fn TaskCard(task_signal: Signal<Task>) -> Element {
    let task_read = task_signal.read();
    let title = task_read.title.as_str();
    let priority = &task_signal.read().priority;

    let handle_click = move |_| {
        task_signal
            .write()
            .update_title("You have changed the title!");
    };
    let change_priority = move |_| {
      task_signal.write().toggle_priority();
    };
    info!("Re-Rendierng-card");
    rsx! {
        div { class: "bg-white my-2 p-4 rounded shadow",
            h3 { onclick: handle_click, "{title}" }
            button {
                class: "rounded border-2 border-gray-500 px-1 py-0.5 text-xs font-medium cursor-pointer",
                onclick: change_priority,
                "{priority}"
            }
        }
    }
}
