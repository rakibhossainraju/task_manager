mod button;
mod input;

use dioxus::prelude::*;
use input::Input;

use crate::{
    components::filter::button::{Button, FilterButtonType},
    models::{add_task_group, use_task_manager},
};

const COLUMN_SVG: &str = include_str!("../../../assets/icons/column.svg");
const LIST_SVG: &str = include_str!("../../../assets/icons/list.svg");
const PLUS_SVG: &str = include_str!("../../../assets/icons/plus.svg");

#[component]
pub fn Filter() -> Element {
    let ctx = use_task_manager();
    let mut task_groups = ctx.task_groups;
    let handle_add_task_group = move |_| {
        task_groups.with_mut(|state| {
            add_task_group(state, "Test Group");
        });
    };
    rsx! {
        div { class: "flex w-full gap-3.5",
            Input {}
            Button { button_type: FilterButtonType::Default,
                span { "Discard" }
            }
            Button { button_type: FilterButtonType::Success,
                span { "Save" }
            }
            div { class: "flex gap-4",
                Button { button_type: FilterButtonType::Outline,
                    span { dangerous_inner_html: COLUMN_SVG }
                }
                Button { button_type: FilterButtonType::Outline,
                    span { dangerous_inner_html: LIST_SVG }
                }
                Button {
                    button_type: FilterButtonType::Outline,
                    onclick: handle_add_task_group,
                    div { class: "flex items-center justify-center gap-2 min-w-[155px]",
                        span { dangerous_inner_html: PLUS_SVG }
                        span { class: "text-slate-400", "Add New Column" }
                    }
                }
            }
        }
    }
}
