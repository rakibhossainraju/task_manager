use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::task_manager::EditingTask,
    models::{BoardState, TaskPriority},
};

#[component]
pub fn EditTaskModal(task_id: Uuid) -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let mut editing_task = use_context::<Signal<EditingTask>>();

    let board_read = board.read();
    let task_info = board_read.groups.iter().find_map(|g| {
        g.tasks.iter().find(|t| t.id == task_id).map(|t| (g.id, t.clone()))
    });

    let Some((initial_group_id, current_task)) = task_info else {
        editing_task.set(EditingTask(None));
        return rsx! {};
    };

    let groups = &board_read.groups;
    let mut title = use_signal(|| current_task.title.clone());
    let mut description = use_signal(|| current_task.description.clone().unwrap_or_default());
    let mut priority = use_signal(|| current_task.priority.clone());
    let mut selected_group_id = use_signal(|| initial_group_id);
    let mut error_msg = use_signal(|| None::<String>);

    let created_at_str = current_task.created_at.format("%b %d, %Y · %H:%M").to_string();
    let updated_at_str = current_task
        .updated_at
        .map(|dt| dt.format("%b %d, %Y · %H:%M").to_string());

    let mut save_task = move || {
        let title_val = title.read().trim().to_string();
        if title_val.is_empty() {
            error_msg.set(Some("Task title cannot be empty".to_string()));
            return;
        }

        let desc_val = description.read().trim().to_string();
        let desc_opt = if desc_val.is_empty() {
            None
        } else {
            Some(desc_val)
        };

        match board.write().update_task(
            task_id,
            title_val,
            desc_opt,
            priority(),
            Some(selected_group_id()),
        ) {
            Ok(_) => {
                editing_task.set(EditingTask(None));
            }
            Err(e) => {
                error_msg.set(Some(e.to_string()));
            }
        }
    };

    let mut delete_this_task = move || {
        board.write().delete_task(task_id);
        editing_task.set(EditingTask(None));
    };

    let handle_close = move |_| {
        editing_task.set(EditingTask(None));
    };

    let handle_backdrop_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Escape {
            editing_task.set(EditingTask(None));
        }
    };

    let handle_title_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Enter {
            save_task();
        }
    };

    let handle_desc_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Enter && (e.modifiers().ctrl() || e.modifiers().meta()) {
            save_task();
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
                        h3 { class: "text-lg font-semibold text-white", "Edit Task" }
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

                // Description Textarea
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-semibold text-[#8798B0] uppercase tracking-wider", "Description" }
                    textarea {
                        class: "w-full bg-[#14161A] text-[#F3F4F6] text-sm rounded-[5px] px-3.5 py-2 outline-none border border-[#2B303A] focus:border-[#00BC7D] transition-colors resize-none h-24 placeholder:text-[#525B6A]",
                        placeholder: "Add details or context...",
                        value: "{description}",
                        onkeydown: handle_desc_keydown,
                        oninput: move |e| description.set(e.data.value()),
                    }
                }

                // Status Column & Priority Row
                div { class: "grid grid-cols-2 gap-3",
                    // Status Column Select
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-semibold text-[#8798B0] uppercase tracking-wider", "Status Column" }
                        select {
                            class: "w-full bg-[#14161A] text-[#F3F4F6] text-sm rounded-[5px] px-3 py-2 outline-none border border-[#2B303A] focus:border-[#00BC7D] cursor-pointer",
                            value: "{selected_group_id}",
                            onchange: move |e| {
                                if let Ok(parsed_id) = Uuid::parse_str(&e.data.value()) {
                                    selected_group_id.set(parsed_id);
                                }
                            },
                            for group in groups.iter() {
                                option {
                                    value: "{group.id}",
                                    selected: group.id == selected_group_id(),
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

                // Timestamps Metadata
                div { class: "flex items-center gap-3 text-xs text-[#8798B0]/70 py-1 border-t border-[#2B303A]/50",
                    span { "Created: {created_at_str}" }
                    if let Some(up_str) = updated_at_str {
                        span { "·" }
                        span { "Updated: {up_str}" }
                    }
                }

                // Error Message Display
                if let Some(err) = error_msg() {
                    span { class: "text-red-400 text-xs font-medium -mt-1", "{err}" }
                }

                // Footer Actions
                div { class: "flex items-center justify-between pt-3 border-t border-[#2B303A]",
                    // Delete Button
                    button {
                        class: "px-3 py-2 text-sm font-medium text-[#E7000B] hover:text-white hover:bg-[#E7000B]/20 rounded-[4px] transition-colors cursor-pointer flex items-center gap-1.5",
                        r#type: "button",
                        onclick: move |_| delete_this_task(),
                        title: "Delete task permanently",
                        svg {
                            width: "14",
                            height: "14",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
                            line { x1: "10", y1: "11", x2: "10", y2: "17" }
                            line { x1: "14", y1: "11", x2: "14", y2: "17" }
                        }
                        span { "Delete" }
                    }

                    // Cancel & Save Buttons
                    div { class: "flex items-center gap-3",
                        button {
                            class: "px-4 py-2 text-sm text-[#8798B0] hover:text-[#F3F4F6] hover:bg-[#22252A] rounded-[4px] transition-colors cursor-pointer",
                            r#type: "button",
                            onclick: handle_close,
                            "Cancel"
                        }
                        button {
                            class: "px-5 py-2 text-sm font-medium text-white bg-[#00BC7D] hover:bg-[#00D492] rounded-[4px] shadow-sm transition-colors cursor-pointer flex items-center gap-1.5",
                            r#type: "button",
                            onclick: move |_| save_task(),
                            svg {
                                width: "14",
                                height: "14",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                            span { "Save Changes" }
                        }
                    }
                }
            }
        }
    }
}
