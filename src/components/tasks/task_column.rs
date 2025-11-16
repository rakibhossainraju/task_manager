use dioxus::prelude::*;

use crate::{
    components::TaskCard,
    models::{update_group, use_task_manager},
};

#[derive(Props, Clone, PartialEq)]
pub struct TaskColumnProps {
    group_id: uuid::Uuid,
}

#[component]
pub fn TaskColumn(props: TaskColumnProps) -> Element {
    let ctx = use_task_manager();
    let mut task_groups = ctx.task_groups;
    let group_id = props.group_id;

    // Read group data from context - extract only what we need (small strings)
    let read_guard = task_groups.read();
    let group = read_guard.iter().find(|g| g.id == group_id);
    let Some(group) = group else {
        return rsx! {
            div { "Group not found" }
        };
    };

    // Clone only small strings, not the entire group
    let title = group.title.clone();
    let description = group.description.clone().unwrap_or_default();
    let task_ids: Vec<uuid::Uuid> = group.task_list.iter().map(|t| t.id).collect();

    drop(read_guard); // Explicitly drop the guard

    let change_group_title = move |_| {
        task_groups.with_mut(|state| {
            update_group(state, group_id, |g| {
                g.update_group_title("New Title!".to_string());
            });
        });
    };

    rsx! {
        div { class: "bg-stone-100 p-4 rounded min-w-[300px]",
            div { class: "bg-stone-200 p-4 rounded",
                h3 { class: "font-medium text-lg", onclick: change_group_title, "{title}" }
                p { "{description}" }
            }
            for task_id in task_ids.iter() {
                TaskCard { task_id: *task_id, group_id, key: "{task_id}" }
            }
        }
    }
}
