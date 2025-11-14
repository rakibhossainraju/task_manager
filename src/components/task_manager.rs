use dioxus::prelude::*;

use crate::{
    components::{Filter, TaskColumn},
    models::{add_task_group, load_task_groups, update_group, update_task},
};

#[component]
pub fn TaskManager() -> Element {
    let mut task_groups = use_signal(|| load_task_groups());

    let handle_add_task_group = move |_| {
        add_task_group(&mut *task_groups.write(), "Test Group");
    };

    let update_group_title = move |(group_id, title): (uuid::Uuid, String)| {
        update_group(&mut *task_groups.write(), group_id, |g| {
            g.update_group_title(title);
        });
    };

    let update_task_title = move |(group_id, task_id, title): (uuid::Uuid, uuid::Uuid, String)| {
        update_task(&mut *task_groups.write(), group_id, task_id, |t| {
            t.update_title(title);
        });
    };

    let toggle_task_priority = move |(group_id, task_id): (uuid::Uuid, uuid::Uuid)| {
        update_task(&mut *task_groups.write(), group_id, task_id, |t| {
            t.toggle_priority();
        });
    };

    rsx! {
        Filter {}
        button {
            class: "bg-blue-500 text-white px-4 py-2 rounded mb-4",
            onclick: handle_add_task_group,
            "+ Add Task Group"
        }
        div { class: "flex gap-4",
            for group in task_groups.read().iter() {
                TaskColumn {
                    group: group.clone(),
                    on_update_title: update_group_title,
                    on_task_title_change: update_task_title,
                    on_toggle_priority: toggle_task_priority,
                }
            }
        }
    }
}
