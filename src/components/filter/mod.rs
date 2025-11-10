mod input;

use dioxus::prelude::*;
use input::Input;

#[component]
pub fn Filter() -> Element {
    rsx! {
        div { class: "bg-primary", Input {} }
    }
}
