use dioxus::prelude::*;

use crate::{
    components::TaskCard,
    models::{TaskGroup, update_group, use_task_manager},
};

#[derive(Props, Clone, PartialEq)]
pub struct TaskColumnProps {
    group: TaskGroup,
}

#[component]
pub fn TaskColumn(props: TaskColumnProps) -> Element {
    let ctx = use_task_manager();
    let mut task_groups = ctx.task_groups;
    let title = &props.group.title;
    let description = props.group.description.as_deref().unwrap_or("");
    let group_id = props.group.id;

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
            for task in props.group.task_list.iter() {
                TaskCard { task: task.clone(), group_id, key: "{task.id}" }
            }
        }
    }
}
