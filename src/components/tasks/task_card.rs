use dioxus::prelude::*;

use crate::models::Task;

#[component]
pub fn TaskCard(task_state: Task) -> Element {
    let task_signal = task_state.task;
    let task_read = task_signal.read();
    let title = task_read.title.as_str();

    let mut task_signal = task_state.task.clone();
    let handle_click = move |_| {
        task_signal
            .write()
            .update_title("You have changed the title!");
    };
    info!("Rendering TaskCard with title: {}", title);
    rsx! {
        div { class: "bg-white my-2 p-4 rounded shadow",
            h3 { onclick: handle_click, "{title}" }
        }
    }
}
