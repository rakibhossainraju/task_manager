use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::{DragContext, TaskCard},
    models::BoardState,
};

#[component]
pub fn TaskColumn(group_id: Uuid) -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let mut drag_state = use_context::<DragContext>();
    let mut is_col_over = use_signal(|| false);

    let board_read = board.read();
    let Some(group) = board_read.groups.iter().find(|g| g.id == group_id) else {
        return rsx! {};
    };
    let title = group.title.as_str();
    let tasks = board_read.filtered_tasks(group_id);
    let count = tasks.len();

    let (outer_color, inner_color) = match title.to_lowercase().as_str() {
        "todo" => ("#00D492", "#004F3B"),
        "progress" => ("#FB64B6", "#8B0836"),
        "done" => ("#C27AFF", "#59168B"),
        "deployed" => ("#FFB900", "#7B3306"),
        _ => ("#8798B0", "#353A44"),
    };

    let col_highlight = if is_col_over() {
        "ring-1 ring-[#00D492]/40"
    } else {
        ""
    };

    rsx! {
        div { class: "flex-1 min-w-[260px] w-full h-full bg-[#191B1F] rounded-[5px] flex flex-col justify-between overflow-hidden shadow-lg border border-[#22252A]/80 {col_highlight}",
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

            // Cards Container & Drop Zone
            div {
                class: "p-2.5 pt-1 pb-3 flex flex-col gap-3 flex-1 overflow-y-auto min-h-0",
                ondragover: move |e| {
                    e.prevent_default();
                    if !is_col_over() {
                        is_col_over.set(true);
                    }
                },
                ondragleave: move |_| {
                    is_col_over.set(false);
                },
                ondrop: move |e| {
                    e.prevent_default();
                    is_col_over.set(false);
                    if let Some(dragged_id) = drag_state() {
                        let target_len = board.read().filtered_tasks(group_id).len();
                        board.write().move_task(dragged_id, group_id, target_len);
                        drag_state.set(None);
                    }
                },
                if count == 0 {
                    div {
                        class: "flex-1 flex flex-col items-center justify-center min-h-[90px] border-2 border-dashed border-[#2D3139]/40 rounded-[10px] text-center select-none",
                        p { class: "text-xs text-[#8798B0]/50 font-medium", "Drop tasks here" }
                    }
                }
                for (idx, task) in tasks.iter().enumerate() {
                    TaskCard {
                        task: (*task).clone(),
                        group_id: group_id,
                        index: idx,
                        key: "{task.id}",
                    }
                }
            }
        }
    }
}
