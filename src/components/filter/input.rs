use dioxus::prelude::*;

const SEARCH_ICON: &str = include_str!("../../../assets/icons/search-icon.svg");

#[component]
pub fn Input() -> Element {
    rsx! {
        div { class: "flex items-center gap-4 px-4 py-2 justify-start",
            span { class: "", dangerous_inner_html: SEARCH_ICON }
            div { class: "relative w-full",
                input {
                    class: "px-4 py-3 text-lg",
                    r#type: "text",
                    placeholder: "",
                }
                span { class: "absolute top-1/2 -translate-y-1/2 block text-slate-400 text-lg",
                    "Filter by "
                    b { class: "font-fira-mono font-medium", "priority" }
                    ", "
                    b { class: "font-fira-mono font-medium", "title" }
                    ", or "
                    b { class: "font-fira-mono font-medium", "description" }
                }
            }
        }
    }
}
