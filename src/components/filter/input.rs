use dioxus::prelude::*;

use crate::models::BoardState;

const SEARCH_ICON: &str = include_str!("../../../assets/icons/search-icon.svg");

#[component]
pub fn Input() -> Element {
    let mut board = use_context::<Signal<BoardState>>();
    let search_query = board.read().search_query.clone();

    rsx! {
        div { class: "flex items-center gap-2.5 px-3.5 h-[40px] w-full bg-[#191B1F] rounded-[4px] border border-[#2B303A]/60 shadow-sm focus-within:border-[#8798B0] transition-colors",
            span {
                class: "text-[#8798B0] flex items-center justify-center shrink-0 w-3.5 h-3.5",
                dangerous_inner_html: SEARCH_ICON,
            }
            div { class: "relative w-full text-sm text-[#F3F4F6] flex items-center",
                input {
                    class: "w-full bg-transparent text-[#F3F4F6] text-sm outline-none border-none py-1 placeholder-transparent",
                    r#type: "text",
                    value: "{search_query}",
                    oninput: move |e| {
                        board.write().set_search_query(e.data.value());
                    },
                }
                span {
                    class: {
                        let visibility_class = if search_query.is_empty() {
                            "visible"
                        } else {
                            "invisible"
                        };
                        format!(
                            "absolute left-0 pointer-events-none text-sm text-[#8798B0] select-none {}",
                            visibility_class,
                        )
                    },
                    "Filter by "
                    b { class: "font-fira-mono font-medium text-[#D1D5DB]", "priority" }
                    " or "
                    b { class: "font-fira-mono font-medium text-[#D1D5DB]", "title" }
                    " or "
                    b { class: "font-fira-mono font-medium text-[#D1D5DB]", "description" }
                    "..."
                }
            }
            if !search_query.is_empty() {
                button {
                    class: "text-[#8798B0] hover:text-[#F3F4F6] transition-colors p-1 cursor-pointer shrink-0",
                    title: "Clear search",
                    r#type: "button",
                    onclick: move |_| {
                        board.write().set_search_query(String::new());
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
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }
        }
    }
}
