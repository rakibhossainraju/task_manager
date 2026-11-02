use dioxus::prelude::*;

use crate::models::{BoardState, Task};

#[component]
pub fn TaskCard(task: Task) -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let title = task.title.as_str();
    let priority = &task.priority;
    let description = task.description.as_deref().unwrap_or("This is a short description");
    let (bg, border, text_color) = priority.colors();
    let task_id = task.id;

    rsx! {
        div { class: "group w-full min-h-[116px] bg-[#22252A] hover:bg-[#272B31] rounded-[12px] p-4 flex flex-col justify-between transition-colors cursor-pointer select-none border border-[#2D3139]/40 relative",
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
