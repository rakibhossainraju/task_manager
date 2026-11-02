use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::TaskCard,
    models::{BoardState, TaskPriority},
};

#[component]
pub fn TaskColumn(group_id: Uuid) -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let board_read = board.read();
    let Some(group) = board_read.groups.iter().find(|g| g.id == group_id) else {
        return rsx! {};
    };
    let title = group.title.as_str();
    let tasks = board_read.filtered_tasks(group_id);
    let count = tasks.len();

    let mut is_adding = use_signal(|| false);
    let mut new_title = use_signal(String::new);
    let mut new_desc = use_signal(String::new);
    let mut new_priority = use_signal(|| TaskPriority::Low);
    let mut error_msg = use_signal(|| None::<String>);

    let (outer_color, inner_color) = match title.to_lowercase().as_str() {
        "todo" => ("#00D492", "#004F3B"),
        "progress" => ("#FB64B6", "#8B0836"),
        "done" => ("#C27AFF", "#59168B"),
        "deployed" => ("#FFB900", "#7B3306"),
        _ => ("#8798B0", "#353A44"),
    };

    let handle_add = move |_| {
        let title_val = new_title.read().clone();
        let desc_val = new_desc.read().clone();
        let desc_opt = if desc_val.trim().is_empty() {
            None
        } else {
            Some(desc_val.trim().to_string())
        };

        match board.write().add_task(group_id, title_val, desc_opt, new_priority()) {
            Ok(_) => {
                new_title.set(String::new());
                new_desc.set(String::new());
                new_priority.set(TaskPriority::Low);
                error_msg.set(None);
                is_adding.set(false);
            }
            Err(e) => {
                error_msg.set(Some(e.to_string()));
            }
        }
    };

    let handle_cancel = move |_| {
        new_title.set(String::new());
        new_desc.set(String::new());
        new_priority.set(TaskPriority::Low);
        error_msg.set(None);
        is_adding.set(false);
    };

    rsx! {
        div { class: "flex-1 min-w-[260px] w-full h-full bg-[#191B1F] rounded-[5px] flex flex-col justify-between overflow-hidden shadow-lg border border-[#22252A]/80",
            // Column Header
            div { class: "p-4 pb-3 shrink-0",
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center gap-2.5",
                        // Status double-ring
                        div {
                            class: "w-4 h-4 rounded-full flex items-center justify-center shrink-0",
                            style: "background-color: {outer_color};",
                            div {
                                class: "w-2.5 h-2.5 rounded-full",
                                style: "background-color: {inner_color};",
                            }
                        }
                        // Column Title
                        span { class: "text-white font-semibold text-lg tracking-tight", "{title}" }
                        // Count Badge
                        span { class: "w-5 h-5 rounded-full bg-[#353A44] text-[#F3F4F6] text-xs font-semibold flex items-center justify-center ml-0.5",
                            "{count}"
                        }
                    }

                    // Context Menu
                    button {
                        class: "p-1 text-[#4B4B4B] hover:text-[#8798B0] transition-colors cursor-pointer",
                        title: "Column actions",
                        svg {
                            width: "16",
                            height: "4",
                            view_box: "0 0 16 4",
                            fill: "none",
                            circle { cx: "2", cy: "2", r: "2", fill: "#4B4B4B" }
                            circle { cx: "8", cy: "2", r: "2", fill: "#4B4B4B" }
                            circle { cx: "14", cy: "2", r: "2", fill: "#4B4B4B" }
                        }
                    }
                }

                // Subtitle
                p { class: "text-[#989898] text-sm mt-1.5 font-normal",
                    "{group.description.as_deref().unwrap_or(\"This is a short description\")}"
                }
            }

            // Cards Container
            div { class: "p-2.5 pt-1 flex flex-col gap-3 flex-1 overflow-y-auto min-h-0",
                for task in tasks.iter() {
                    TaskCard { task: (*task).clone(), key: "{task.id}" }
                }

                if is_adding() {
                    div { class: "p-3 bg-[#22252A] rounded-[10px] border border-[#353A44] flex flex-col gap-2.5 shadow-md shrink-0",
                        input {
                            class: "w-full bg-[#191B1F] text-[#F3F4F6] text-sm rounded-[4px] px-2.5 py-1.5 outline-none border border-[#353A44] focus:border-[#8798B0]",
                            placeholder: "Task title...",
                            value: "{new_title}",
                            autofocus: true,
                            oninput: move |e| new_title.set(e.data.value()),
                        }
                        input {
                            class: "w-full bg-[#191B1F] text-[#F3F4F6] text-xs rounded-[4px] px-2.5 py-1.5 outline-none border border-[#353A44] focus:border-[#8798B0]",
                            placeholder: "Description (optional)...",
                            value: "{new_desc}",
                            oninput: move |e| new_desc.set(e.data.value()),
                        }
                        div { class: "flex items-center justify-between gap-2 pt-1",
                            select {
                                class: "bg-[#191B1F] text-[#F3F4F6] text-xs rounded-[4px] px-2 py-1 outline-none border border-[#353A44] cursor-pointer",
                                value: "{new_priority}",
                                onchange: move |e| {
                                    let prio = match e.data.value().as_str() {
                                        "Urgent" => TaskPriority::Urgent,
                                        "High" => TaskPriority::High,
                                        "Mid" | "Medium" => TaskPriority::Medium,
                                        _ => TaskPriority::Low,
                                    };
                                    new_priority.set(prio);
                                },
                                option { value: "Low", "Low" }
                                option { value: "Mid", "Medium" }
                                option { value: "High", "High" }
                                option { value: "Urgent", "Urgent" }
                            }
                            div { class: "flex items-center gap-2",
                                button {
                                    class: "px-2.5 py-1 text-xs text-[#8798B0] hover:text-[#F3F4F6] transition-colors cursor-pointer rounded",
                                    r#type: "button",
                                    onclick: handle_cancel,
                                    "Cancel"
                                }
                                button {
                                    class: "px-3 py-1 text-xs font-medium text-white bg-[#00BC7D] hover:bg-[#00D492] rounded transition-colors cursor-pointer shadow-sm",
                                    r#type: "button",
                                    onclick: handle_add,
                                    "Add"
                                }
                            }
                        }
                        if let Some(err) = error_msg() {
                            span { class: "text-red-400 text-xs", "{err}" }
                        }
                    }
                }
            }

            // Footer "+ Add Item"
            if !is_adding() {
                button {
                    class: "w-full py-2.5 px-4 shrink-0 bg-[#4B4B4B]/40 hover:bg-[#4B4B4B]/60 text-[#8798B0] hover:text-[#F3F4F6] rounded-b-[5px] flex items-center gap-2.5 transition-colors cursor-pointer text-sm font-medium border-t border-[#22252A]/60",
                    onclick: move |_| is_adding.set(true),
                    svg {
                        width: "14",
                        height: "14",
                        view_box: "0 0 14 14",
                        fill: "none",
                        path {
                            d: "M7 1.5V12.5M1.5 7H12.5",
                            stroke: "currentColor",
                            stroke_width: "1.8",
                            stroke_linecap: "round",
                        }
                    }
                    span { "Add Item" }
                }
            }
        }
    }
}
