// use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}
// impl Default for TaskPriority {
//     fn default() -> Self {
//         TaskPriority::Low
//     }
// }
// impl Display for TaskPriority {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let priority_str = match self {
//             TaskPriority::Low => "Low",
//             TaskPriority::Medium => "Medium",
//             TaskPriority::High => "High",
//             TaskPriority::Urgent => "Urgent",
//         };
//         write!(f, "{}", priority_str)
//     }
// }


#[derive(Debug, Props, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub id: u128,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: String,
    pub updated_at: Option<String>,
}
