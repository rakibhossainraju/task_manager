use dioxus::prelude::*;

use crate::models::{TaskManagerContext, load_task_groups};

#[component]
pub fn TaskManagerProvider(children: Element) -> Element {
    let task_groups = use_signal(|| load_task_groups());
    let context = TaskManagerContext {
        task_groups,
    };

    use_context_provider(|| context);

    rsx! {
        {children}
    }
}

