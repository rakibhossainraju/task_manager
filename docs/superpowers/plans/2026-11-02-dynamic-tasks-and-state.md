# Dynamic Tasks and State Handling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the task manager state to eliminate nested signals and make task creation, task deletion, and search filtering dynamic and reactive.

**Architecture:** A unified, idiomatic `BoardState` containing plain Rust domain structs (`TaskGroup`, `Task`) provided at the top level via Dioxus context (`Signal<BoardState>`). Pure business logic methods on `BoardState` handle state mutations and filtering, while components read slices and trigger mutations cleanly without nested signal locks.

**Tech Stack:** Rust 2024, Dioxus 0.7, serde / serde_json, uuid, chrono

**Spec:** `docs/superpowers/specs/2026-11-02-dynamic-tasks-and-state-design.md`

## Global Constraints
- No nested signals (`Signal<Vec<Signal<...>>>` is strictly forbidden).
- Domain types (`Task`, `TaskGroup`, `BoardState`) must be plain Rust structs with `Serialize` and `Deserialize`.
- All mutations must be pure and testable with standard `cargo test`.
- Visual styling must preserve existing Tailwind theme and color schemes.

## Review Focus
1. Empty title validation when adding a task: adding a whitespace-only title should be rejected or trimmed.
2. Deleting a nonexistent task ID: `delete_task` should return `false` gracefully without panicking.
3. Case-insensitive search: search query "URGENT" or "urgent" should match priority and text regardless of casing.
4. Search query with whitespace: leading/trailing whitespace in search input should not break matching.
5. Deleting a task while a search filter is active: the filtered view must immediately reflect the deletion.

---

### Task 1: Clean Domain State and Business Logic (TDD)

**Files:**
- Create: `src/models/board.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/models/task/mod.rs`
- Modify: `src/models/task_group/mod.rs`
- Test: `src/models/board.rs` (unit tests in module)

**Interfaces:**
- Produces:
  - `pub struct Task { pub id: Uuid, pub title: String, pub description: Option<String>, pub priority: TaskPriority, pub created_at: DateTime<Local>, pub updated_at: Option<DateTime<Local>> }`
  - `pub struct TaskGroup { pub id: Uuid, pub title: String, pub description: Option<String>, pub tasks: Vec<Task> }`
  - `pub struct BoardState { pub groups: Vec<TaskGroup>, pub search_query: String }`
  - `BoardState::new() -> Self`
  - `BoardState::add_task(&mut self, group_id: Uuid, title: String, description: Option<String>, priority: TaskPriority) -> Result<Uuid, &'static str>`
  - `BoardState::delete_task(&mut self, task_id: Uuid) -> bool`
  - `BoardState::set_search_query(&mut self, query: String)`
  - `BoardState::filtered_tasks(&self, group_id: Uuid) -> Vec<&Task>`

- [ ] **Step 1: Write the failing tests for `BoardState` operations**

Write unit tests in `src/models/board.rs` covering:
- Creating a board from seeded JSON data.
- Adding a valid task to a group.
- Rejecting empty or whitespace-only task titles.
- Deleting an existing task by ID (returns `true`).
- Deleting a non-existent task by ID (returns `false`).
- Case-insensitive search filtering across title, description, and priority.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test models::board`
Expected: FAIL with compilation error (types/methods not yet defined).

- [ ] **Step 3: Implement clean `Task`, `TaskGroup`, and `BoardState` without nested signals**

Implement plain Rust structs in `src/models/board.rs` and update `src/models/mod.rs`. Ensure serde deserialization from `task_groups.json` works cleanly.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test models::board`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/models/
git commit -m "refactor(models): replace nested signals with clean BoardState and domain models"
```

---

### Task 2: Root Store Context and Component Architecture Refactoring

**Files:**
- Modify: `src/components/task_manager.rs`
- Modify: `src/components/tasks/task_column.rs`
- Modify: `src/components/tasks/task_card.rs`

**Interfaces:**
- Consumes:
  - `BoardState` from `src/models/board.rs`
- Produces:
  - `TaskManager`: provides `Signal<BoardState>` context.
  - `TaskColumn(group_id: Uuid)`: reads group and filtered tasks from context.
  - `TaskCard(task: Task)`: renders task props without inner signal locks.

- [ ] **Step 1: Write verification test for model integration**

Verify `BoardState::load_default()` successfully parses `task_groups.json` and loads all groups and tasks without signals.

- [ ] **Step 2: Run test to verify it passes**

Run: `cargo test test_board_state_initial_load`
Expected: PASS

- [ ] **Step 3: Update `TaskManager`, `TaskColumn`, and `TaskCard`**

Refactor `TaskManager` to provide `use_context_provider(|| Signal::new(BoardState::new()))`.
Refactor `TaskColumn` to take `group_id: Uuid`, query `board.read().filtered_tasks(group_id)`, and render `TaskCard` passing `task: task.clone()`.
Remove dead nested signal references and fix component props.

- [ ] **Step 4: Verify project compiles cleanly**

Run: `cargo check`
Expected: PASS with 0 errors.

- [ ] **Step 5: Commit**

```bash
git add src/components/
git commit -m "refactor(components): connect TaskManager context and simplify column/card props"
```

---

### Task 3: Task Creation (Inline Form in Task Column)

**Files:**
- Modify: `src/components/tasks/task_column.rs`

**Interfaces:**
- Consumes:
  - `board.write().add_task(group_id, title, description, priority)`

- [ ] **Step 1: Write unit test for adding task with different priorities**

Test `add_task` with Low, Medium, High, and Urgent priorities in `src/models/board.rs`.

- [ ] **Step 2: Run test to verify it passes**

Run: `cargo test test_add_task_priorities`
Expected: PASS

- [ ] **Step 3: Implement inline creation form in `TaskColumn`**

In `src/components/tasks/task_column.rs`:
- Add state `is_adding = use_signal(|| false)`.
- Add draft state for title, description, and selected priority.
- Clicking "+ Add Item" opens the form.
- Form includes text inputs, priority dropdown/selector, "Add" button, and "Cancel" button.
- Submitting trims title, calls `board.write().add_task(...)`, resets form, and closes inline view.

- [ ] **Step 4: Verify compilation and tests**

Run: `cargo check && cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/components/tasks/task_column.rs
git commit -m "feat(tasks): implement inline task creation form in TaskColumn"
```

---

### Task 4: Task Deletion (TaskCard Action)

**Files:**
- Modify: `src/components/tasks/task_card.rs`

**Interfaces:**
- Consumes:
  - `board.write().delete_task(task_id: Uuid)`

- [ ] **Step 1: Write unit test for task deletion**

Test removing task from a group and verifying it is no longer returned in `filtered_tasks`.

- [ ] **Step 2: Run test to verify it passes**

Run: `cargo test test_delete_task`
Expected: PASS

- [ ] **Step 3: Implement delete button on `TaskCard`**

In `src/components/tasks/task_card.rs`:
- Add a delete icon button (trash icon svg) in the card header next to priority badge.
- Clicking delete triggers `board.write().delete_task(task.id)`.
- Use `e.stop_propagation()` so card clicks don't conflict.

- [ ] **Step 4: Verify compilation and tests**

Run: `cargo check && cargo test`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/components/tasks/task_card.rs
git commit -m "feat(tasks): add delete task action on TaskCard"
```

---

### Task 5: Live Search Filtering

**Files:**
- Modify: `src/components/filter/mod.rs`
- Modify: `src/components/filter/input.rs`

**Interfaces:**
- Consumes:
  - `board.read().search_query`
  - `board.write().set_search_query(query: String)`

- [ ] **Step 1: Write unit tests for search query scenarios**

Add tests for:
- Matching title substring.
- Matching description substring.
- Matching priority name.
- Empty query returning all tasks.
- Non-matching query returning empty list.

- [ ] **Step 2: Run test to verify it passes**

Run: `cargo test test_search_filtering`
Expected: PASS

- [ ] **Step 3: Connect search `Input` to board context**

In `src/components/filter/input.rs`:
- Read `search_query` from `use_context::<Signal<BoardState>>()`.
- Update `board.write().set_search_query(e.data.value())` on input change.
- Show clear button ("x") when search query is not empty to quickly clear filter.

- [ ] **Step 4: Verify full project compilation and test suite**

Run: `cargo test && cargo check`
Expected: All tests pass, 0 warnings/errors.

- [ ] **Step 5: Commit**

```bash
git add src/components/filter/
git commit -m "feat(filter): bind search input to BoardState query for live filtering"
```
