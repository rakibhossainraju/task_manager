use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::DragContext,
    models::{BoardState, Task},
};

#[component]
pub fn TaskCard(task: Task, group_id: Uuid) -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let mut drag_state = use_context::<DragContext>();
    let mut is_over = use_signal(|| false);

    let title = task.title.as_str();
    let priority = &task.priority;
    let description = task.description.as_deref().unwrap_or("This is a short description");
    let (bg, border, text_color) = priority.colors();
    let task_id = task.id;
    let is_dragging = drag_state() == Some(task_id);
    let is_drop_target = is_over() && drag_state().is_some() && drag_state() != Some(task_id);

    let over_classes = if is_drop_target {
        "border-t-2 border-t-[#00D492] ring-2 ring-[#00D492]/20"
    } else {
        ""
    };

    let drag_classes = if is_dragging {
        "opacity-40 scale-[0.98] border-dashed border-[#8798B0]/60"
    } else {
        "hover:bg-[#272B31]"
    };

    rsx! {
        div {
            class: "group w-full min-h-[116px] bg-[#22252A] rounded-[12px] p-4 flex flex-col justify-between transition-all cursor-grab active:cursor-grabbing select-none border border-[#2D3139]/40 relative {drag_classes} {over_classes}",
            draggable: "true",
            ondragstart: move |e| {
                e.stop_propagation();
                drag_state.set(Some(task_id));
            },
            ondragend: move |_| {
                drag_state.set(None);
                is_over.set(false);
            },
            ondragover: move |e| {
                e.prevent_default();
                e.stop_propagation();
                if drag_state() != Some(task_id) && !is_over() {
                    is_over.set(true);
                }
            },
            ondragleave: move |_| {
                is_over.set(false);
            },
            ondrop: move |e| {
                e.prevent_default();
                e.stop_propagation();
                is_over.set(false);
                if let Some(dragged_id) = drag_state() {
                    board.write().move_task(dragged_id, group_id, Some(task_id));
                    drag_state.set(None);
                }
            },
            div { class: "flex items-center justify-between gap-2",
                h4 { class: "text-white font-semibold text-base tracking-tight truncate flex-1", "{title}" }
                div { class: "flex items-center gap-1.5 shrink-0",
                    span {
                        class: "h-[18px] px-2 rounded-lg text-xs font-semibold flex items-center justify-center shrink-0 border",
                        style: "background-color: {bg}; border-color: {border}; color: {text_color};",
                        "{priority}"
                    }
                    button {
                        class: "opacity-0 group-hover:opacity-100 p-1 text-[#8798B0] hover:text-[#E7000B] hover:bg-[#353A44]/50 rounded transition-all cursor-pointer",
                        title: "Delete task",
                        r#type: "button",
                        onclick: move |e| {
                            e.stop_propagation();
                            board.write().delete_task(task_id);
                        },
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
                    }
                }
            }
            div { class: "mt-3 text-[#989898] text-sm leading-relaxed space-y-1",
                p { class: "truncate", "{description}" }
            }
        }
    }
}
