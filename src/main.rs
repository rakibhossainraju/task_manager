mod components;
mod models;

use components::TaskManager;
use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use dioxus::desktop;
        let window_builder = desktop::WindowBuilder::new()
            .with_always_on_top(true)
            .with_title("Task Manager");

        dioxus::LaunchBuilder::new()
            .with_cfg(
                desktop::Config::default()
                    .with_menu(None)
                    .with_window(window_builder),
            )
            .launch(App);
    }
    #[cfg(target_arch = "wasm32")]
    {
        launch(App);
    }
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
