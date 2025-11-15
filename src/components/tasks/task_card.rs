use dioxus::prelude::*;

use crate::models::{Task, update_task, use_task_manager};

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    task: Task,
    group_id: uuid::Uuid,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    let ctx = use_task_manager();
    let mut task_groups = ctx.task_groups;
    let title = &props.task.title;
    let priority = &props.task.priority;
    let group_id = props.group_id;
    let task_id = props.task.id;

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
