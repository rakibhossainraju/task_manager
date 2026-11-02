# Drag and Drop Task Movement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement native Drag & Drop for task cards to reorder tasks within columns and transfer them between columns with immediate status updates.

**Architecture:** A domain method `BoardState::move_task` handles pure index calculation and column transfers. A root-level `Signal<Option<Uuid>>` drag context in `TaskManager` coordinates the currently dragged card with card and column drag/drop event listeners.

**Tech Stack:** Rust 2024, Dioxus 0.7, HTML5 Drag & Drop API, Tailwind CSS

**Spec:** `docs/superpowers/specs/2026-11-02-drag-and-drop-design.md`

## Global Constraints
- Pure domain logic with comprehensive unit tests for `move_task`.
- Zero compiler warnings or errors (`cargo check && cargo test`).
- Zero extra external crate dependencies.
- Native HTML5 drag/drop events.

## Review Focus
1. Moving task to same position: moving a card to its current index in the same group should be a no-op that leaves the list intact.
2. Dropping into empty column: dropping onto a column with 0 tasks appends the card without index out-of-bounds panics.
3. Target index bounds clamping: if `target_index` exceeds destination task count, it safely appends.
4. Drag cancellation: releasing the drag outside drop zones clears the drag state cleanly without mutations.
5. Nonexistent task/group safety: invalid UUIDs return `false` without panicking.

---

### Task 1: Domain Logic for Task Reordering & Inter-Column Movement (TDD)

**Files:**
- Modify: `src/models/board.rs`
- Test: `src/models/board.rs` (unit tests)

**Interfaces:**
- Produces:
  - `BoardState::move_task(&mut self, task_id: Uuid, target_group_id: Uuid, target_index: usize) -> bool`

- [ ] **Step 1: Write failing unit tests for `move_task`**

Write tests in `src/models/board.rs` covering:
- Intra-column reordering (e.g. moving task from index 0 to 2 in group 0).
- Inter-column movement (e.g. moving task from group 0 to group 1 at index 0).
- Clamping when target_index is larger than destination length.
- Handling nonexistent task or group gracefully (returns `false`).

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test test_move_task`
Expected: FAIL with compilation error (`move_task` method not yet defined).

- [ ] **Step 3: Implement `move_task` in `BoardState`**

In `src/models/board.rs`:
- Find and remove `task` by `task_id` from its current group (recording old position).
- If not found, return `false`.
- Find target group by `target_group_id`. If not found, restore task to original group and return `false`.
- Clamp `target_index` to `target_group.tasks.len()`.
- Insert task at `target_index`.
- Set `task.updated_at = Some(Local::now())`.
- Return `true`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test test_move_task`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/models/board.rs
git commit -m "feat(board): implement move_task domain method for drag-and-drop reordering"
```

---

### Task 2: Drag State Context & Card-Level Drag-and-Drop

**Files:**
- Modify: `src/components/task_manager.rs`
- Modify: `src/components/tasks/task_card.rs`

**Interfaces:**
- Produces:
  - `pub type DragContext = Signal<Option<Uuid>>;`
  - `TaskCard(task: Task, group_id: Uuid, index: usize)`: supports drag and drop.

- [ ] **Step 1: Update `TaskCard` props and add drag context in `TaskManager`**

In `src/components/task_manager.rs`:
- Define `pub type DragContext = Signal<Option<Uuid>>;`.
- Provide `use_context_provider(|| Signal::new(None::<Uuid>));`.

In `src/components/tasks/task_card.rs`:
- Add `group_id: Uuid` and `index: usize` to `TaskCard` props.
- Add `draggable: "true"`.
- Read `drag_state = use_context::<Signal<Option<Uuid>>>()`.
- Add local `is_over = use_signal(|| false)`.
- `ondragstart: move |e| { e.stop_propagation(); drag_state.set(Some(task_id)); }`.
- `ondragend: move |_| { drag_state.set(None); is_over.set(false); }`.
- `ondragover: move |e| { e.prevent_default(); is_over.set(true); }`.
- `ondragleave: move |_| { is_over.set(false); }`.
- `ondrop: move |e| { e.prevent_default(); e.stop_propagation(); is_over.set(false); if let Some(dragged_id) = drag_state() { board.write().move_task(dragged_id, group_id, index); drag_state.set(None); } }`.
- Visual styling for `is_dragging` (reduced opacity) and `is_over` (top emerald border highlight).

- [ ] **Step 2: Update `TaskColumn` to pass `group_id` and `index` to `TaskCard`**

In `src/components/tasks/task_column.rs`:
- Pass `group_id: group_id, index: idx` in loop `for (idx, task) in tasks.iter().enumerate()`.

- [ ] **Step 3: Verify compilation and tests**

Run: `cargo check && cargo test`
Expected: PASS with 0 errors.

- [ ] **Step 4: Commit**

```bash
git add src/components/task_manager.rs src/components/tasks/task_card.rs src/components/tasks/task_column.rs
git commit -m "feat(tasks): connect card drag events and drop targets with visual indicators"
```

---

### Task 3: Column Container Drop Zone & End-to-End Suite Verification

**Files:**
- Modify: `src/components/tasks/task_column.rs`

**Interfaces:**
- Consumes:
  - `DragContext`
  - `board.write().move_task(dragged_id, group_id, tasks.len())`

- [ ] **Step 1: Implement column container drop zone**

In `src/components/tasks/task_column.rs`:
- Add `ondragover: move |e| { e.prevent_default(); }`.
- Add `ondrop: move |e| { e.prevent_default(); if let Some(dragged_id) = drag_state() { let target_len = board.read().filtered_tasks(group_id).len(); board.write().move_task(dragged_id, group_id, target_len); drag_state.set(None); } }`.

- [ ] **Step 2: Verify full test suite and compiler output**

Run: `cargo check && cargo test`
Expected: All tests pass, 0 warnings/errors.

- [ ] **Step 3: Commit**

```bash
git add src/components/tasks/task_column.rs
git commit -m "feat(tasks): add column-level drop zone for empty areas"
```
