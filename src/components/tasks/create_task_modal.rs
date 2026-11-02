use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{BoardState, TaskPriority};

#[component]
pub fn CreateTaskModal(mut is_open: Signal<bool>) -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut priority = use_signal(|| TaskPriority::Low);
    let mut selected_group_id = use_signal(|| None::<Uuid>);
    let mut error_msg = use_signal(|| None::<String>);

    let board_read = board.read();
    let groups = &board_read.groups;

    // Default to first group if none selected
    let current_group_id = selected_group_id().or_else(|| groups.first().map(|g| g.id));

    let mut submit_task = move || {
        let title_val = title.read().trim().to_string();
        if title_val.is_empty() {
            error_msg.set(Some("Task title is required".to_string()));
            return;
        }

        let Some(target_group_id) = current_group_id else {
            error_msg.set(Some("No column selected".to_string()));
            return;
        };

        let desc_val = description.read().trim().to_string();
        let desc_opt = if desc_val.is_empty() {
            None
        } else {
            Some(desc_val)
        };

        match board
            .write()
            .add_task(target_group_id, title_val, desc_opt, priority())
        {
            Ok(_) => {
                title.set(String::new());
                description.set(String::new());
                priority.set(TaskPriority::Low);
                error_msg.set(None);
                is_open.set(false);
            }
            Err(e) => {
                error_msg.set(Some(e.to_string()));
            }
        }
    };

    let handle_close = move |_| {
        error_msg.set(None);
        is_open.set(false);
    };

    let handle_backdrop_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Escape {
            error_msg.set(None);
            is_open.set(false);
        }
    };

    let handle_title_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Enter {
            submit_task();
        }
    };

    let handle_desc_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Enter && (e.modifiers().ctrl() || e.modifiers().meta()) {
            submit_task();
        }
    };

    rsx! {
        // Fullscreen Backdrop
        div {
            class: "fixed inset-0 bg-black/70 backdrop-blur-xs flex items-center justify-center z-50 p-4 select-none outline-none",
            tabindex: "0",
            onclick: handle_close,
            onkeydown: handle_backdrop_keydown,
            // Modal Card Box
            div {
                class: "bg-[#191B1F] border border-[#2B303A] rounded-[10px] w-full max-w-lg shadow-2xl p-6 flex flex-col gap-4 text-[#F3F4F6] relative",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "flex items-center justify-between pb-2 border-b border-[#2B303A]",
                    div { class: "flex items-center gap-2",
                        div { class: "w-2.5 h-2.5 rounded-full bg-[#00BC7D]" }
                        h3 { class: "text-lg font-semibold text-white", "Create New Task" }
                    }
                    button {
                        class: "p-1.5 text-[#8798B0] hover:text-white rounded-[4px] hover:bg-[#22252A] transition-colors cursor-pointer",
                        title: "Close (Esc)",
                        onclick: handle_close,
                        svg {
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }

                // Title Input
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-semibold text-[#8798B0] uppercase tracking-wider",
                        "Task Title "
                        span { class: "text-red-400", "*" }
                    }
                    input {
                        class: "w-full bg-[#14161A] text-[#F3F4F6] text-sm rounded-[5px] px-3.5 py-2.5 outline-none border border-[#2B303A] focus:border-[#00BC7D] transition-colors placeholder:text-[#525B6A]",
                        placeholder: "What needs to be done?",
                        value: "{title}",
                        autofocus: true,
                        onkeydown: handle_title_keydown,
                        oninput: move |e| {
                            title.set(e.data.value());
                            if error_msg().is_some() {
                                error_msg.set(None);
                            }
                        },
                    }
                }

                // Description Input
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-semibold text-[#8798B0] uppercase tracking-wider", "Description" }
                    textarea {
                        class: "w-full bg-[#14161A] text-[#F3F4F6] text-sm rounded-[5px] px-3.5 py-2 outline-none border border-[#2B303A] focus:border-[#00BC7D] transition-colors resize-none h-20 placeholder:text-[#525B6A]",
                        placeholder: "Add details or context (optional)...",
                        value: "{description}",
                        onkeydown: handle_desc_keydown,
                        oninput: move |e| description.set(e.data.value()),
                    }
                }

                // Priority & Status Row
                div { class: "grid grid-cols-2 gap-3",
                    // Status Column Select
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-semibold text-[#8798B0] uppercase tracking-wider", "Status Column" }
                        select {
                            class: "w-full bg-[#14161A] text-[#F3F4F6] text-sm rounded-[5px] px-3 py-2 outline-none border border-[#2B303A] focus:border-[#00BC7D] cursor-pointer",
                            value: "{current_group_id.map(|id| id.to_string()).unwrap_or_default()}",
                            onchange: move |e| {
                                if let Ok(parsed_id) = Uuid::parse_str(&e.data.value()) {
                                    selected_group_id.set(Some(parsed_id));
                                }
                            },
                            for group in groups.iter() {
                                option {
                                    value: "{group.id}",
                                    selected: Some(group.id) == current_group_id,
                                    "{group.title}"
                                }
                            }
                        }
                    }

                    // Priority Select
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-semibold text-[#8798B0] uppercase tracking-wider", "Priority" }
                        select {
                            class: "w-full bg-[#14161A] text-[#F3F4F6] text-sm rounded-[5px] px-3 py-2 outline-none border border-[#2B303A] focus:border-[#00BC7D] cursor-pointer",
                            value: "{priority}",
                            onchange: move |e| {
                                let prio = match e.data.value().as_str() {
                                    "Urgent" => TaskPriority::Urgent,
                                    "High" => TaskPriority::High,
                                    "Mid" | "Medium" => TaskPriority::Medium,
                                    _ => TaskPriority::Low,
                                };
                                priority.set(prio);
                            },
                            option { value: "Low", "Low" }
                            option { value: "Mid", "Medium" }
                            option { value: "High", "High" }
                            option { value: "Urgent", "Urgent" }
                        }
                    }
                }

                // Error Message Display
                if let Some(err) = error_msg() {
                    span { class: "text-red-400 text-xs font-medium -mt-1", "{err}" }
                }

                // Footer Actions
                div { class: "flex items-center justify-end gap-3 pt-3 border-t border-[#2B303A]",
                    button {
                        class: "px-4 py-2 text-sm text-[#8798B0] hover:text-[#F3F4F6] hover:bg-[#22252A] rounded-[4px] transition-colors cursor-pointer",
                        r#type: "button",
                        onclick: handle_close,
                        "Cancel"
                    }
                    button {
                        class: "px-5 py-2 text-sm font-medium text-white bg-[#00BC7D] hover:bg-[#00D492] rounded-[4px] shadow-sm transition-colors cursor-pointer flex items-center gap-1.5",
                        r#type: "button",
                        onclick: move |_| submit_task(),
                        svg {
                            width: "14",
                            height: "14",
                            view_box: "0 0 14 14",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            path { d: "M7 1V13M1 7H13" }
                        }
                        span { "Create Task" }
                    }
                }
            }
        }
    }
}
