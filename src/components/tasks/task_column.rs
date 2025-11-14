use dioxus::prelude::*;

use crate::{components::TaskCard, models::TaskGroup};

#[derive(Props, Clone, PartialEq)]
pub struct TaskColumnProps {
    group: TaskGroup,
    #[props(default)]
    on_update_title: Option<EventHandler<(uuid::Uuid, String)>>,
    #[props(default)]
    on_task_title_change: Option<EventHandler<(uuid::Uuid, uuid::Uuid, String)>>,
    #[props(default)]
    on_toggle_priority: Option<EventHandler<(uuid::Uuid, uuid::Uuid)>>,
}

#[component]
pub fn TaskColumn(props: TaskColumnProps) -> Element {
    let title = &props.group.title;
    let description = props.group.description.as_deref().unwrap_or("");

    let change_group_title = move |_| {
        if let Some(handler) = &props.on_update_title {
            handler.call((props.group.id, "New Title!".to_string()));
        }
    };

    rsx! {
        div { class: "bg-stone-100 my-2 p-4 rounded min-w-[300px]",
            div { class: "bg-stone-200 p-4 rounded",
                h3 { class: "font-medium text-lg", onclick: change_group_title, "{title}" }
                p { "{description}" }
            }
            for task in props.group.task_list.iter() {
                TaskCard { 
                    task: task.clone(),
                    group_id: props.group.id,
                    on_title_change: props.on_task_title_change,
                    on_toggle_priority: props.on_toggle_priority,
                    key: "{task.id}"
                }
            }
        }
    }
}
