# Modal Task Creation & Toolbar Cleanup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace per-column inline creation forms and obsolete toolbar buttons with a clean, root-level modal dialog triggered from a new "Add Task" button in the top toolbar.

**Architecture:** A new `CreateTaskModal` component rendered at the root level of `TaskManager` to avoid CSS stacking and overflow clipping. State is triggered via an open/close signal connected to an "Add Task" button in `Filter`, and submits new tasks into the chosen column via the existing `BoardState::add_task` method.

**Tech Stack:** Rust 2024, Dioxus 0.7, Tailwind CSS

**Spec:** `docs/superpowers/specs/2026-11-02-modal-task-creation-design.md`

## Global Constraints
- Preserve existing Tailwind styling conventions and color palettes.
- Do not reintroduce nested signals; use existing root `BoardState` context.
- Keep unit tests green and compile with 0 warnings.

## Review Focus
1. Default column selection: when opening the modal, the status dropdown must default to a valid group ID (first group).
2. Title validation: submitting with an empty or whitespace title must show an error and not close the modal.
3. Keyboard accessibility: pressing <kbd>Escape</kbd> anywhere in the modal must close it; pressing <kbd>Enter</kbd> must submit.
4. Backdrop click: clicking outside the modal content box must close the modal.
5. Column list responsiveness: if columns change, the status dropdown dynamically reflects active columns.

---

### Task 1: Clean Up Column Footers & Remove Inline Creation

**Files:**
- Modify: `src/components/tasks/task_column.rs`

**Interfaces:**
- Produces:
  - `TaskColumn(group_id: Uuid)`: Clean component without inline forms or footer buttons.

- [ ] **Step 1: Inspect `TaskColumn` and prepare changes**

Verify existing tests pass before refactoring `TaskColumn`:
Run: `cargo test`
Expected: PASS

- [ ] **Step 2: Remove inline creation states and "+ Add Item" footer**

In `src/components/tasks/task_column.rs`:
- Remove `is_adding`, `new_title`, `new_desc`, `new_priority`, `error_msg`.
- Remove `submit_task`, `cancel_task`, and `handle_keydown`.
- Remove the inline creation JSX and the "+ Add Item" footer button.
- Retain column header, description, and the task cards list.

- [ ] **Step 3: Verify compilation and tests**

Run: `cargo check && cargo test`
Expected: PASS with 0 errors.

- [ ] **Step 4: Commit**

```bash
git add src/components/tasks/task_column.rs
git commit -m "refactor(tasks): remove inline creation form and footer from TaskColumn"
```

---

### Task 2: Implement `CreateTaskModal` Component

**Files:**
- Create: `src/components/tasks/create_task_modal.rs`
- Modify: `src/components/tasks/mod.rs`

**Interfaces:**
- Produces:
  - `pub fn CreateTaskModal(is_open: Signal<bool>) -> Element`

- [ ] **Step 1: Create `src/components/tasks/create_task_modal.rs`**

Implement `CreateTaskModal`:
- Read `BoardState` from context.
- Form inputs: Title, Description, Priority dropdown (`TaskPriority`), Status dropdown (populated from `board.read().groups`).
- Set default status to `board.read().groups.first().map(|g| g.id)`.
- Fullscreen backdrop with `fixed inset-0 bg-black/60 flex items-center justify-center z-50`.
- Backdrop click closes modal (`is_open.set(false)`).
- Modal content box stops event propagation.
- <kbd>Escape</kbd> key closes modal; <kbd>Enter</kbd> key submits.
- On valid submit, calls `board.write().add_task(...)`, resets form, and sets `is_open.set(false)`.

- [ ] **Step 2: Export `CreateTaskModal` in `src/components/tasks/mod.rs`**

- [ ] **Step 3: Verify compilation and tests**

Run: `cargo check && cargo test`
Expected: PASS with 0 errors.

- [ ] **Step 4: Commit**

```bash
git add src/components/tasks/create_task_modal.rs src/components/tasks/mod.rs
git commit -m "feat(tasks): create CreateTaskModal component with status and priority selection"
```

---

### Task 3: Update Toolbar and Connect Modal in `TaskManager`

**Files:**
- Modify: `src/components/filter/mod.rs`
- Modify: `src/components/task_manager.rs`
- Modify: `src/components/mod.rs`

**Interfaces:**
- Consumes:
  - `CreateTaskModal` from `src/components/tasks/create_task_modal.rs`
- Produces:
  - `Filter(on_add_task: EventHandler)`: Toolbar with "+ Add Task" button replacing Discard/Save.
  - `TaskManager`: Mounts `CreateTaskModal` and controls `is_create_modal_open`.

- [ ] **Step 1: Update `Filter` in `src/components/filter/mod.rs`**

- Remove "Discard" and "Save" buttons.
- Add "+ Add Task" button styled with `#00BC7D` accent:
  ```rust
  button {
      class: "h-[39px] px-4 bg-[#00BC7D] hover:bg-[#00D492] text-white text-sm font-medium rounded-[4px] shadow-sm transition-colors cursor-pointer flex items-center gap-2",
      onclick: move |_| on_add_task.call(()),
      svg { /* plus icon */ }
      span { "Add Task" }
  }
  ```

- [ ] **Step 2: Connect `CreateTaskModal` in `TaskManager`**

In `src/components/task_manager.rs`:
- Add `let mut is_create_modal_open = use_signal(|| false);`.
- Pass `on_add_task: move |_| is_create_modal_open.set(true)` to `Filter`.
- If `is_create_modal_open()`, render `CreateTaskModal { is_open: is_create_modal_open }`.

- [ ] **Step 3: Verify full suite compilation and tests**

Run: `cargo check && cargo test`
Expected: All tests pass, 0 warnings/errors.

- [ ] **Step 4: Commit**

```bash
git add src/components/filter/mod.rs src/components/task_manager.rs src/components/mod.rs
git commit -m "feat(ui): add Add Task toolbar button and connect CreateTaskModal at root level"
```
