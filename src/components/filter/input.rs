use dioxus::prelude::*;

const SEARCH_ICON: &str = include_str!("../../../assets/icons/search-icon.svg");

#[component]
pub fn Input() -> Element {
    let mut input_value = use_signal(String::new);

    rsx! {
        div { class: "flex items-center justify-start gap-2.5 px-4.5 bg-primary drop-shadow-default rounded border border-transparent focus-within:border-slate-300 w-full",
            span { dangerous_inner_html: SEARCH_ICON }
            div { class: "relative w-full text-slate-400 text-lg",
                input {
                    class: "w-full  py-2.5 text-lg outline-none border-none",
                    r#type: "text",
                    oninput: move |e| {
                        input_value.set(e.data.value());
                    },
                }
                span {
                    class: {
                        let visibility_class: &str = if input_value.read().is_empty() {
                            "visible"
                        } else {
                            "invisible"
                        };
                        format!(
                            "absolute top-1/2 -translate-y-1/2 block pointer-events-none {}",
                            visibility_class,
                        )
                    },
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
