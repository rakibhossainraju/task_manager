mod input;

use dioxus::prelude::*;
pub use input::Input;

#[component]
pub fn Filter(on_add_task: EventHandler) -> Element {
    rsx! {
        div { class: "w-full px-[23.5px] py-5 flex items-center justify-between gap-4 flex-wrap",
            // Search Input
            div { class: "flex-1 min-w-[280px] max-w-xl lg:max-w-2xl",
                Input {}
            }

            // Controls & Action Buttons
            div { class: "flex items-center gap-3",
                div { class: "flex items-center gap-2",
                    // Kanban Board View Button (Active)
                    button {
                        class: "w-[46px] h-[39px] bg-[#191B1F] border border-[#8798B0] rounded-[4px] flex items-center justify-center text-[#F3F4F6] hover:bg-[#22252A] transition-colors cursor-pointer",
                        title: "Board View",
                        svg {
                            width: "18",
                            height: "18",
                            view_box: "0 0 18 18",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            rect { x: "1", y: "1", width: "16", height: "16", rx: "2" }
                            line { x1: "6", y1: "1", x2: "6", y2: "17" }
                            line { x1: "12", y1: "1", x2: "12", y2: "17" }
                        }
                    }
                    // List View Button
                    button {
                        class: "w-[46px] h-[39px] bg-[#191B1F] border border-[#353A44] rounded-[4px] flex items-center justify-center text-[#8798B0] hover:text-[#F3F4F6] hover:border-[#8798B0] transition-colors cursor-pointer",
                        title: "List View",
                        svg {
                            width: "18",
                            height: "18",
                            view_box: "0 0 18 18",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            line { x1: "2", y1: "3", x2: "2.01", y2: "3" }
                            line { x1: "2", y1: "9", x2: "2.01", y2: "9" }
                            line { x1: "2", y1: "15", x2: "2.01", y2: "15" }
                            line { x1: "6", y1: "3", x2: "16", y2: "3" }
                            line { x1: "6", y1: "9", x2: "16", y2: "9" }
                            line { x1: "6", y1: "15", x2: "16", y2: "15" }
                        }
                    }
                }

                // Add New Column Button
                button {
                    class: "w-[164px] h-[39px] bg-[#191B1F] border border-[#8798B0] hover:border-white rounded-[4px] flex items-center justify-center gap-2 text-[#F3F4F6] text-sm font-medium transition-colors cursor-pointer",
                    svg {
                        width: "14",
                        height: "14",
                        view_box: "0 0 14 14",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.8",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M7 1V13M1 7H13" }
                    }
                    span { "Add New Column" }
                }

                // Add Task Button (placed right after Add New Column)
                button {
                    class: "h-[39px] px-4 bg-[#00BC7D] hover:bg-[#00D492] text-white text-sm font-medium rounded-[4px] shadow-sm transition-colors cursor-pointer flex items-center gap-2",
                    r#type: "button",
                    onclick: move |_| on_add_task.call(()),
                    svg {
                        width: "14",
                        height: "14",
                        view_box: "0 0 14 14",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        path { d: "M7 1V13M1 7H13" }
                    }
                    span { "Add Task" }
                }
            }
        }
    }
}
