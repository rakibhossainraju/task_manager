mod components;
mod task_manager;

use dioxus::prelude::*;
use task_manager::TaskManager;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        TaskManager {}
    }
}
