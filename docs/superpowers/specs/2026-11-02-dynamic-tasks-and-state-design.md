# Dynamic Tasks and State Handling Design Specification

## Overview & Intent
Transform the task manager from a static UI with nested signal antipatterns into a fully interactive, reactive Kanban board. The focus of this phase is clean state architecture, adding tasks, removing tasks, and reactive search filtering across columns.

## Goals
1. **Clean State Management**: Eliminate the nested `Signal<Vec<Signal<TaskGroup>>>` and `Signal<Vec<Signal<Task>>>` hierarchy. Replace with a single, unified `BoardState` containing plain Rust structs (`Vec<TaskGroup>` and `Vec<Task>`), managed via Dioxus context.
2. **Task Creation**: Allow users to add a new task to any column with a title, optional description, and priority (Low, Medium, High, Urgent).
3. **Task Deletion**: Allow users to remove any task from any column with immediate UI updates.
4. **Reactive Search Filter**: Filter visible tasks across all columns in real-time as the user types in the filter bar (matching title, description, or priority).
5. **Robustness & Testability**: Provide unit tests verifying state operations (creation, deletion, search filtering) independently of the UI.

## Non-Goals
- Drag-and-drop between columns (deferred to next phase).
- Multi-user collaboration or backend sync.
- Editing column names / deleting columns (UI exists for column actions, but core dynamic functionality focuses on tasks).
- List view toggle implementation (retains board view focus for now).

## Approaches Considered

### Approach 1: Normalized Flat Store with HashMaps
- Store `tasks: HashMap<Uuid, Task>` and `groups: Vec<TaskGroupRef>` where `TaskGroupRef` holds `task_ids: Vec<Uuid>`.
- *Pros*: O(1) task lookups and deletes.
- *Cons*: More boilerplate, harder JSON serialization/deserialization with current data shape, overkill for typical Kanban board sizes (< 500 tasks).

### Approach 2: Clean Hierarchical Domain Store with Context Signal (Recommended)
- Plain Rust structs: `BoardState` containing `groups: Vec<TaskGroup>`, where each `TaskGroup` contains `tasks: Vec<Task>`.
- Root state exposed via Dioxus context: `Signal<BoardState>`.
- Pure business logic methods on `BoardState` for adding, deleting, and filtering tasks.
- *Pros*: Idiomatic Dioxus 0.7 pattern, completely eliminates nested signals, serde-compatible with existing JSON schema, trivial to test with standard Rust `cargo test`.
- *Cons*: Deleting a task by ID searches groups, which is negligible for Kanban boards.

### Approach 3: Fine-grained Signals per Field
- Keep nested signals but use helper wrappers.
- *Rejected*: Explicitly rejected by user ("The current task gruop and statei is a signal mess. I don't want you to resue the current signal mess.").

## Architecture & Data Models

### Data Structures (`src/models/board.rs` or `src/models/`):
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardState {
    pub groups: Vec<TaskGroup>,
    #[serde(skip)]
    pub search_query: String,
}
```

### Core Operations on `BoardState`:
- `new()` / `default()`: loads seeded data from `task_groups.json` into clean structs.
- `add_task(&mut self, group_id: Uuid, title: String, description: Option<String>, priority: TaskPriority) -> Uuid`
- `delete_task(&mut self, task_id: Uuid) -> bool`
- `set_search_query(&mut self, query: String)`
- `filtered_tasks(&self, group_id: Uuid) -> Vec<&Task>`: returns tasks matching `search_query` (case-insensitive substring match against title, description, and priority name).

## UI Interaction Design

1. **Inline Task Creation inside `TaskColumn`**:
   - Clicking "+ Add Item" at the column footer toggles an inline creation form.
   - Form includes: Title input, Description input (optional), Priority selector dropdown/buttons, "Add" and "Cancel" buttons.
   - Submitting validates title (non-empty), calls `board.write().add_task(...)`, resets input, and closes inline form.
   - Pressing "Cancel" or Escape cancels creation.

2. **Task Deletion in `TaskCard`**:
   - Each `TaskCard` includes a subtle trash/delete button (visible on hover or neatly placed next to the priority badge).
   - Clicking delete calls `board.write().delete_task(task.id)`.

3. **Search & Filter in `Filter` / `Input`**:
   - The search input binds directly to `board.read().search_query` and updates `board.write().set_search_query(...)` on input change.
   - If search query is non-empty, a clear ("x") button appears to easily reset the query.
   - Each `TaskColumn` displays filtered tasks and shows current filtered count.
