use dioxus::prelude::*;

#[component]
pub fn TaskManager() -> Element {
    rsx! {
        div {
            h1 {
                class: "",
                "Task Manager Component"
            }
        }
    }
}
