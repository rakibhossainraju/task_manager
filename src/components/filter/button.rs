use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum FilterButtonType {
    Default,
    Success,
    Outline,
}
impl FilterButtonType {
    pub fn class(&self) -> &str {
        match self {
            FilterButtonType::Default => "bg-primary hover:bg-primary/70",
            FilterButtonType::Success => "bg-emerald-500 hover:bg-emerald-400",
            FilterButtonType::Outline => {
                "bg-primary border border-slate-400 text-slate-400 hover:bg-slate-300/10"
            }
        }
    }
}
#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    pub button_type: FilterButtonType,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_class = props.button_type.class();
    let onclick = move |e| {
        if let Some(handler) = &props.onclick {
            handler.call(e);
        }
    };
    rsx! {
        button {
            class: "text-white font-medium rounded drop-shadow-default px-4.5 py-3 cursor-pointer {button_class}",
            onclick,
            {props.children}
        }
    }
}
