use crate::models::TaskGroup;
use crate::models::task_group::TaskGroupsData;
use uuid::Uuid;

pub type TasksGroupsState = Vec<TaskGroup>;

pub fn load_task_groups() -> TasksGroupsState {
    let json_str = include_str!("./task_groups.json");
    let groups_data: TaskGroupsData =
        serde_json::from_str(json_str).expect("Failed to parse task_groups.json");

    groups_data.into_iter().map(TaskGroup::from).collect()
}

pub fn add_task_group(state: &mut TasksGroupsState, group_name: impl Into<String>) {
    let group = TaskGroup {
        id: Uuid::new_v4(),
        title: group_name.into(),
        description: Some("This is description".to_string()),
        task_list: Vec::new(),
    };
    state.push(group);
}

pub fn update_group(state: &mut TasksGroupsState, group_id: uuid::Uuid, f: impl FnOnce(&mut TaskGroup)) {
    if let Some(group) = state.iter_mut().find(|g| g.id == group_id) {
        f(group);
    }
}

pub fn update_task(state: &mut TasksGroupsState, group_id: uuid::Uuid, task_id: uuid::Uuid, f: impl FnOnce(&mut crate::models::Task)) {
    if let Some(group) = state.iter_mut().find(|g| g.id == group_id) {
        group.update_task(task_id, f);
    }
}
