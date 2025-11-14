use dioxus::prelude::*;

use crate::models::Task;

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    task: Task,
    group_id: uuid::Uuid,
    #[props(default)]
    on_title_change: Option<EventHandler<(uuid::Uuid, uuid::Uuid, String)>>,
    #[props(default)]
    on_toggle_priority: Option<EventHandler<(uuid::Uuid, uuid::Uuid)>>,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    let title = &props.task.title;
    let priority = &props.task.priority;

    let handle_click = move |_| {
        if let Some(handler) = &props.on_title_change {
            handler.call((props.group_id, props.task.id, "You have changed the title!".to_string()));
        }
    };
    
    let change_priority = move |_| {
        if let Some(handler) = &props.on_toggle_priority {
            handler.call((props.group_id, props.task.id));
        }
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
