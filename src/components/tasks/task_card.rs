use dioxus::prelude::*;

use crate::models::Task;

#[component]
pub fn TaskCard(task: Task) -> Element {
    let title = task.title.as_str();
    let priority = &task.priority;
    let description = task.description.as_deref().unwrap_or("This is a short description");
    let (bg, border, text_color) = priority.colors();

    rsx! {
        div { class: "w-full min-h-[116px] bg-[#22252A] hover:bg-[#272B31] rounded-[12px] p-4 flex flex-col justify-between transition-colors cursor-pointer select-none border border-[#2D3139]/40",
            div { class: "flex items-center justify-between gap-2",
                h4 { class: "text-white font-semibold text-base tracking-tight truncate", "{title}" }
                span {
                    class: "h-[18px] px-2 rounded-lg text-xs font-semibold flex items-center justify-center shrink-0 border",
                    style: "background-color: {bg}; border-color: {border}; color: {text_color};",
                    "{priority}"
                }
            }
            div { class: "mt-3 text-[#989898] text-sm leading-relaxed space-y-1",
                p { class: "truncate", "{description}" }
            }
        }
    }
}
