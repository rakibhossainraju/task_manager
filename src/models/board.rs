use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::task::{Task, TaskPriority};
use crate::models::task_group::TaskGroup;

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

    pub fn update_task(
        &mut self,
        task_id: Uuid,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
        target_group_id: Option<Uuid>,
    ) -> Result<(), &'static str> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err("Task title cannot be empty");
        }

        // Find task across all groups
        let mut source_info = None;
        for (g_idx, group) in self.groups.iter().enumerate() {
            if let Some(t_idx) = group.tasks.iter().position(|t| t.id == task_id) {
                source_info = Some((g_idx, t_idx));
                break;
            }
        }

        let Some((source_g_idx, source_t_idx)) = source_info else {
            return Err("Task not found");
        };

        let current_group_id = self.groups[source_g_idx].id;

        // If target_group_id is specified and differs from current group
        if let Some(target_gid) = target_group_id {
            if target_gid != current_group_id {
                let target_exists = self.groups.iter().any(|g| g.id == target_gid);
                if !target_exists {
                    return Err("Target group not found");
                }

                let mut task = self.groups[source_g_idx].tasks.remove(source_t_idx);
                task.title = trimmed_title.to_string();
                task.description = description;
                task.priority = priority;
                task.updated_at = Some(Local::now());

                let target_group = self
                    .groups
                    .iter_mut()
                    .find(|g| g.id == target_gid)
                    .expect("target group confirmed to exist");

                target_group.tasks.push(task);
                return Ok(());
            }
        }

        // Otherwise update in-place
        let task = &mut self.groups[source_g_idx].tasks[source_t_idx];
        task.title = trimmed_title.to_string();
        task.description = description;
        task.priority = priority;
        task.updated_at = Some(Local::now());
        Ok(())
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

    pub fn move_task(
        &mut self,
        task_id: Uuid,
        target_group_id: Uuid,
        target_task_id: Option<Uuid>,
    ) -> bool {
        // Dropping onto itself is a safe no-op
        if target_task_id == Some(task_id) {
            return true;
        }

        let mut source_info = None;
        for (g_idx, group) in self.groups.iter().enumerate() {
            if let Some(t_idx) = group.tasks.iter().position(|t| t.id == task_id) {
                source_info = Some((g_idx, t_idx));
                break;
            }
        }

        let Some((source_g_idx, source_t_idx)) = source_info else {
            return false;
        };

        let target_group_opt = self.groups.iter().find(|g| g.id == target_group_id);
        let Some(target_group) = target_group_opt else {
            return false;
        };

        // If target_task_id is specified, verify it exists in target_group
        if let Some(target_tid) = target_task_id {
            if !target_group.tasks.iter().any(|t| t.id == target_tid) {
                return false;
            }
        }

        let mut task = self.groups[source_g_idx].tasks.remove(source_t_idx);
        task.updated_at = Some(Local::now());

        let target_group = self
            .groups
            .iter_mut()
            .find(|g| g.id == target_group_id)
            .expect("target group confirmed to exist");

        if let Some(target_tid) = target_task_id {
            let insert_pos = target_group
                .tasks
                .iter()
                .position(|t| t.id == target_tid)
                .unwrap_or(target_group.tasks.len());
            target_group.tasks.insert(insert_pos, task);
        } else {
            target_group.tasks.push(task);
        }

        true
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
                let priority_matches = match task.priority {
                    TaskPriority::Low => "low".contains(&query),
                    TaskPriority::Medium => "medium".contains(&query) || "mid".contains(&query),
                    TaskPriority::High => "high".contains(&query),
                    TaskPriority::Urgent => "urgent".contains(&query),
                };

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
        let total_initial = board.groups[0].tasks.len();

        board
            .add_task(
                group_id,
                "UniqueSearchKeyword".to_string(),
                Some("Special Description Text".to_string()),
                TaskPriority::Urgent,
            )
            .unwrap();

        // Title substring match (case-insensitive)
        board.set_search_query("  uNiQuEsEaRcH  ".to_string());
        let filtered_title = board.filtered_tasks(group_id);
        assert_eq!(filtered_title.len(), 1);
        assert_eq!(filtered_title[0].title, "UniqueSearchKeyword");

        // Description substring match
        board.set_search_query("description text".to_string());
        let filtered_desc = board.filtered_tasks(group_id);
        assert_eq!(filtered_desc.len(), 1);
        assert_eq!(filtered_desc[0].title, "UniqueSearchKeyword");

        // Priority name match
        board.set_search_query("URGENT".to_string());
        let filtered_prio = board.filtered_tasks(group_id);
        assert!(filtered_prio.iter().any(|t| t.title == "UniqueSearchKeyword"));

        // Empty query returns all tasks in group
        board.set_search_query("   ".to_string());
        let filtered_all = board.filtered_tasks(group_id);
        assert_eq!(filtered_all.len(), total_initial + 1);

        // No match returns empty
        board.set_search_query("nonexistentxyz123".to_string());
        let filtered_none = board.filtered_tasks(group_id);
        assert_eq!(filtered_none.len(), 0);
    }

    #[test]
    fn test_search_filtering_medium_priority() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        board
            .add_task(
                group_id,
                "Feature Alpha Task".to_string(),
                None,
                TaskPriority::Medium,
            )
            .unwrap();

        board.set_search_query("medium".to_string());
        assert!(
            board
                .filtered_tasks(group_id)
                .iter()
                .any(|t| t.title == "Feature Alpha Task")
        );

        board.set_search_query("mid".to_string());
        assert!(
            board
                .filtered_tasks(group_id)
                .iter()
                .any(|t| t.title == "Feature Alpha Task")
        );
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

    #[test]
    fn test_move_task_intra_column_downward() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        let task0_id = board.groups[0].tasks[0].id;
        let task1_id = board.groups[0].tasks[1].id;
        let task2_id = board.groups[0].tasks[2].id;
        let task3_id = board.groups[0].tasks[3].id;

        // Move task 0 before task 2 (downward drag)
        let success = board.move_task(task0_id, group_id, Some(task2_id));
        assert!(success);
        assert_eq!(board.groups[0].tasks.len(), 4);
        assert_eq!(board.groups[0].tasks[0].id, task1_id);
        assert_eq!(board.groups[0].tasks[1].id, task0_id);
        assert_eq!(board.groups[0].tasks[2].id, task2_id);
        assert_eq!(board.groups[0].tasks[3].id, task3_id);
    }

    #[test]
    fn test_move_task_intra_column_upward() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        let task0_id = board.groups[0].tasks[0].id;
        let task1_id = board.groups[0].tasks[1].id;
        let task2_id = board.groups[0].tasks[2].id;
        let task3_id = board.groups[0].tasks[3].id;

        // Move task 3 before task 1 (upward drag)
        let success = board.move_task(task3_id, group_id, Some(task1_id));
        assert!(success);
        assert_eq!(board.groups[0].tasks.len(), 4);
        assert_eq!(board.groups[0].tasks[0].id, task0_id);
        assert_eq!(board.groups[0].tasks[1].id, task3_id);
        assert_eq!(board.groups[0].tasks[2].id, task1_id);
        assert_eq!(board.groups[0].tasks[3].id, task2_id);
    }

    #[test]
    fn test_move_task_inter_column() {
        let mut board = BoardState::new();
        let source_group_id = board.groups[0].id;
        let target_group_id = board.groups[1].id;
        let task_id = board.groups[0].tasks[0].id;
        let target_first_task_id = board.groups[1].tasks[0].id;
        let source_initial_len = board.filtered_tasks(source_group_id).len();
        let target_initial_len = board.filtered_tasks(target_group_id).len();

        let success = board.move_task(task_id, target_group_id, Some(target_first_task_id));
        assert!(success);
        assert_eq!(board.filtered_tasks(source_group_id).len(), source_initial_len - 1);
        assert_eq!(board.filtered_tasks(target_group_id).len(), target_initial_len + 1);
        assert_eq!(board.groups[1].tasks[0].id, task_id);
        assert!(board.groups[1].tasks[0].updated_at.is_some());
    }

    #[test]
    fn test_move_task_empty_column_and_append() {
        let mut board = BoardState::new();
        let target_group_id = board.groups[3].id;
        let task_id = board.groups[0].tasks[0].id;

        // Explicitly clear group 3 to test true empty column drop
        board.groups[3].tasks.clear();
        assert_eq!(board.groups[3].tasks.len(), 0);

        let success = board.move_task(task_id, target_group_id, None);
        assert!(success);
        assert_eq!(board.groups[3].tasks.len(), 1);
        assert_eq!(board.groups[3].tasks[0].id, task_id);
    }

    #[test]
    fn test_move_task_nonexistent_ids() {
        let mut board = BoardState::new();
        let real_group_id = board.groups[0].id;
        let real_task_id = board.groups[0].tasks[0].id;
        let fake_id = Uuid::new_v4();

        // Nonexistent source task
        assert!(!board.move_task(fake_id, real_group_id, None));

        // Nonexistent target group (task must NOT be lost from source group)
        let initial_tasks_count = board.groups[0].tasks.len();
        assert!(!board.move_task(real_task_id, fake_id, None));
        assert_eq!(board.groups[0].tasks.len(), initial_tasks_count);
        assert_eq!(board.groups[0].tasks[0].id, real_task_id);

        // Nonexistent target task in real target group
        assert!(!board.move_task(real_task_id, real_group_id, Some(fake_id)));
    }

    #[test]
    fn test_move_task_same_position() {
        let mut board = BoardState::new();
        let group_id = board.groups[0].id;
        let task_id = board.groups[0].tasks[1].id;

        // Moving onto itself is a safe no-op
        assert!(board.move_task(task_id, group_id, Some(task_id)));
        assert_eq!(board.groups[0].tasks[1].id, task_id);
    }

    #[test]
    fn test_update_task_success() {
        let mut board = BoardState::new();
        let task_id = board.groups[0].tasks[0].id;

        let res = board.update_task(
            task_id,
            "Updated Title".to_string(),
            Some("Updated Description".to_string()),
            TaskPriority::Urgent,
            None,
        );
        assert!(res.is_ok());

        let task = board.groups[0].tasks.iter().find(|t| t.id == task_id).unwrap();
        assert_eq!(task.title, "Updated Title");
        assert_eq!(task.description.as_deref(), Some("Updated Description"));
        assert_eq!(task.priority, TaskPriority::Urgent);
        assert!(task.updated_at.is_some());
    }

    #[test]
    fn test_update_task_group_movement() {
        let mut board = BoardState::new();
        let target_group_id = board.groups[1].id;
        let task_id = board.groups[0].tasks[0].id;
        let source_len = board.groups[0].tasks.len();
        let target_len = board.groups[1].tasks.len();

        let res = board.update_task(
            task_id,
            "Moved Task".to_string(),
            None,
            TaskPriority::High,
            Some(target_group_id),
        );
        assert!(res.is_ok());

        assert_eq!(board.groups[0].tasks.len(), source_len - 1);
        assert_eq!(board.groups[1].tasks.len(), target_len + 1);
        let moved = board.groups[1].tasks.iter().find(|t| t.id == task_id).unwrap();
        assert_eq!(moved.title, "Moved Task");
        assert_eq!(moved.priority, TaskPriority::High);
    }

    #[test]
    fn test_update_task_validation_and_errors() {
        let mut board = BoardState::new();
        let task_id = board.groups[0].tasks[0].id;
        let fake_id = Uuid::new_v4();

        // Empty title
        let empty_res = board.update_task(
            task_id,
            "   ".to_string(),
            None,
            TaskPriority::Low,
            None,
        );
        assert!(empty_res.is_err());

        // Nonexistent task
        let fake_task_res = board.update_task(
            fake_id,
            "Valid Title".to_string(),
            None,
            TaskPriority::Low,
            None,
        );
        assert!(fake_task_res.is_err());

        // Nonexistent target group
        let fake_group_res = board.update_task(
            task_id,
            "Valid Title".to_string(),
            None,
            TaskPriority::Low,
            Some(fake_id),
        );
        assert!(fake_group_res.is_err());
    }
}
