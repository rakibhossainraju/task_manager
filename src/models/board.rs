use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::TaskPriority;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(title: String, description: Option<String>, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            priority,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "task_list", alias = "tasks")]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BoardState {
    pub groups: Vec<TaskGroup>,
    #[serde(skip)]
    pub search_query: String,
}

impl BoardState {
    pub fn new() -> Self {
        Self::load_default()
    }

    pub fn load_default() -> Self {
        let json_str = include_str!("./task_group/task_groups.json");
        let groups: Vec<TaskGroup> =
            serde_json::from_str(json_str).expect("Failed to parse task_groups.json");

        Self {
            groups,
            search_query: String::new(),
        }
    }

    pub fn add_task(
        &mut self,
        group_id: Uuid,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
    ) -> Result<Uuid, &'static str> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err("Task title cannot be empty");
        }

        let group = self
            .groups
            .iter_mut()
            .find(|g| g.id == group_id)
            .ok_or("Group not found")?;

        let task = Task::new(trimmed_title.to_string(), description, priority);
        let task_id = task.id;
        group.tasks.push(task);
        Ok(task_id)
    }

    pub fn delete_task(&mut self, task_id: Uuid) -> bool {
        for group in &mut self.groups {
            if let Some(pos) = group.tasks.iter().position(|t| t.id == task_id) {
                group.tasks.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn filtered_tasks(&self, group_id: Uuid) -> Vec<&Task> {
        let Some(group) = self.groups.iter().find(|g| g.id == group_id) else {
            return Vec::new();
        };

        let query = self.search_query.trim().to_lowercase();
        if query.is_empty() {
            return group.tasks.iter().collect();
        }

        group
            .tasks
            .iter()
            .filter(|task| {
                let title_matches = task.title.to_lowercase().contains(&query);
                let desc_matches = task
                    .description
                    .as_deref()
                    .map(|d| d.to_lowercase().contains(&query))
                    .unwrap_or(false);
                let priority_str = format!("{}", task.priority).to_lowercase();
                let priority_matches = priority_str.contains(&query);

                title_matches || desc_matches || priority_matches
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_state_initial_load() {
        let board = BoardState::new();
        assert_eq!(board.groups.len(), 4);
        assert_eq!(board.groups[0].title, "Todo");
        assert_eq!(board.groups[0].tasks.len(), 4);
    }

    #[test]
    fn test_add_task_success_and_validation() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        let initial_count = board.groups[0].tasks.len();

        let res = board.add_task(
            group_id,
            "New Feature".to_string(),
            Some("Desc".to_string()),
            TaskPriority::High,
        );
        assert!(res.is_ok());
        assert_eq!(board.groups[0].tasks.len(), initial_count + 1);

        // Reject empty or whitespace title
        let empty_res = board.add_task(group_id, "   ".to_string(), None, TaskPriority::Low);
        assert!(empty_res.is_err());
    }

    #[test]
    fn test_delete_task() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        let task_id = board
            .add_task(
                group_id,
                "To Delete".to_string(),
                None,
                TaskPriority::Low,
            )
            .unwrap();

        assert!(board.filtered_tasks(group_id).iter().any(|t| t.id == task_id));
        assert!(board.delete_task(task_id));
        assert!(!board.filtered_tasks(group_id).iter().any(|t| t.id == task_id));
        assert!(!board.delete_task(task_id)); // second delete returns false
    }

    #[test]
    fn test_search_filtering() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        board
            .add_task(
                group_id,
                "UniqueSearchKeyword".to_string(),
                None,
                TaskPriority::Urgent,
            )
            .unwrap();

        board.set_search_query("uniquesearch".to_string());
        let filtered = board.filtered_tasks(group_id);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "UniqueSearchKeyword");

        // Filter by priority
        board.set_search_query("urgent".to_string());
        let filtered_prio = board.filtered_tasks(group_id);
        assert!(filtered_prio.iter().any(|t| t.title == "UniqueSearchKeyword"));

        // No match
        board.set_search_query("nonexistentxyz123".to_string());
        let filtered_none = board.filtered_tasks(group_id);
        assert_eq!(filtered_none.len(), 0);
    }

    #[test]
    fn test_add_task_priorities() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;

        for prio in [
            TaskPriority::Low,
            TaskPriority::Medium,
            TaskPriority::High,
            TaskPriority::Urgent,
        ] {
            let res = board.add_task(
                group_id,
                format!("Task {:?}", prio),
                Some("Test desc".to_string()),
                prio.clone(),
            );
            assert!(res.is_ok());
            let task_id = res.unwrap();
            let added_task = board.groups[0].tasks.iter().find(|t| t.id == task_id).unwrap();
            assert_eq!(added_task.priority, prio);
        }
    }
}
