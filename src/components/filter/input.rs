use dioxus::prelude::*;

const SEARCH_ICON: &str = include_str!("../../../assets/icons/search-icon.svg");

#[component]
pub fn Input() -> Element {
    let mut input_value = use_signal(String::new);

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
                    value: "{input_value}",
                    oninput: move |e| {
                        input_value.set(e.data.value());
                    },
                }
                span {
                    class: {
                        let visibility_class = if input_value.read().is_empty() {
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
        }
    }
}
