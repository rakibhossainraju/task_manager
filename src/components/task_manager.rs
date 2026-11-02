use dioxus::prelude::*;

use crate::{
    components::{Filter, TaskColumn},
    models::BoardState,
};

#[component]
pub fn TaskManager() -> Element {
    let board = use_context_provider(|| Signal::new(BoardState::new()));

    rsx! {
        div { class: "w-full h-screen bg-[#0E1116] flex flex-col font-sans select-none overflow-hidden",
            // Action & Filter Toolbar
            Filter {}

            // Kanban Columns Board Container
            main { class: "flex-1 px-6 pb-6 overflow-hidden w-full flex flex-col min-h-0",
                div { class: "flex items-stretch gap-[19px] w-full flex-1 overflow-x-auto min-h-0",
                    for group in board.read().groups.iter() {
                        TaskColumn { group_id: group.id, key: "{group.id}" }
                    }
                }
            }
        }
    }
}
