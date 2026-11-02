use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::{CreateTaskModal, EditTaskModal, Filter, TaskColumn},
    models::BoardState,
};

pub type DragContext = Signal<Option<Uuid>>;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct EditingTask(pub Option<Uuid>);

#[component]
pub fn TaskManager() -> Element {
    let board = use_context_provider(|| Signal::new(BoardState::new()));
    let _drag_context = use_context_provider(|| Signal::new(None::<Uuid>));
    let editing_task = use_context_provider(|| Signal::new(EditingTask(None)));
    let mut is_create_modal_open = use_signal(|| false);

    rsx! {
        div { class: "w-full h-screen bg-[#0E1116] flex flex-col font-sans select-none overflow-hidden relative",
            // Action & Filter Toolbar
            Filter {
                on_add_task: move |_| is_create_modal_open.set(true),
            }

            // Kanban Columns Board Container
            main { class: "flex-1 px-6 pb-6 overflow-hidden w-full flex flex-col min-h-0",
                div { class: "flex items-stretch gap-[19px] w-full flex-1 overflow-x-auto min-h-0",
                    for group in board.read().groups.iter() {
                        TaskColumn { group_id: group.id, key: "{group.id}" }
                    }
                }
            }

            // Root-Level Create Task Modal
            if is_create_modal_open() {
                CreateTaskModal { is_open: is_create_modal_open }
            }

            // Root-Level Edit Task Modal
            if let Some(task_id) = editing_task().0 {
                EditTaskModal { task_id, key: "{task_id}" }
            }
        }
    }
}
