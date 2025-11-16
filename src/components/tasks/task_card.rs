use dioxus::prelude::*;

use crate::models::{update_task, use_task_manager};

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    task_id: uuid::Uuid,
    group_id: uuid::Uuid,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    let ctx = use_task_manager();
    let mut task_groups = ctx.task_groups;
    let group_id = props.group_id;
    let task_id = props.task_id;

    // Read task data from context - extract only what we need (small strings)
    let read_guard = task_groups.read();
    let task = read_guard
        .iter()
        .find(|g| g.id == group_id)
        .and_then(|g| g.task_list.iter().find(|t| t.id == task_id));

    let Some(task) = task else {
        return rsx! { div { "Task not found" } };
    };

    // Clone only small strings, not the entire task
    let title = task.title.clone();
    let priority = task.priority.clone();

    drop(read_guard); // Explicitly drop the guard

    let handle_click = move |_| {
        task_groups.with_mut(|state| {
            update_task(state, group_id, task_id, |t| {
                t.update_title("You have changed the title!".to_string());
            });
        });
    };

    let mut task_groups2 = task_groups;
    let change_priority = move |_| {
        task_groups2.with_mut(|state| {
            update_task(state, group_id, task_id, |t| {
                t.toggle_priority();
            });
        });
    };

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
