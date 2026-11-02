# Task Detail & Edit Modal Design Specification

## Overview & Intent
Provide full task inspection and editing capabilities on the Kanban board. Clicking any task card opens a centered modal dialog displaying all task details, allowing users to update the title, description, priority, and column status, view creation/update timestamps, or delete the task.

## Goals
1. **Domain Logic (`BoardState::update_task`)**:
   - Add `pub fn update_task(&mut self, task_id: Uuid, title: String, description: Option<String>, priority: TaskPriority, target_group_id: Option<Uuid>) -> Result<(), &'static str>` to `BoardState`.
   - Validates that title is not empty after trimming.
   - Updates title, description, and priority on the task.
   - Sets `task.updated_at = Some(Local::now())`.
   - If `target_group_id` is provided and differs from the current group, moves the task to the target group.
   - Comprehensive unit test coverage for updates, status moves, empty title validation, and nonexistent IDs.

2. **Modal UI (`EditTaskModal`)**:
   - Centered dark modal dialog matching the aesthetic and styling of `CreateTaskModal`.
   - Backdrop click dismiss and <kbd>Escape</kbd> key dismiss.
   - <kbd>Enter</kbd> submit on title input, <kbd>Ctrl+Enter</kbd> / <kbd>Cmd+Enter</kbd> on description textarea.
   - Dynamic status column dropdown selector (populated from `board.groups`).
   - Interactive priority buttons (`Low`, `Medium`, `High`, `Urgent`) with active color borders and backgrounds.
   - Display task creation timestamp and last updated timestamp (if available).
   - "Delete Task" button with safe removal.

3. **Context Coordination**:
   - Root-level `EditingTask(pub Option<Uuid>)` context provided by `TaskManager`.
   - Clicking a `TaskCard` triggers `editing_task.set(EditingTask(Some(task_id)))`.
   - Preserves stop-propagation on card delete button and drag handlers.

## Non-Goals
- Comment threads / activity logs on tasks (future milestone).
- Rich text / markdown live preview editor (plain text multiline is sufficient).
- File attachments.

## Approaches Considered
- **Approach 1: Centered Edit Modal (Selected)**
  - Reuses design patterns, keyboard navigation, and responsive styles from `CreateTaskModal`.
  - Cleanest UX for quick focused edits.
- **Approach 2: Slide-Over Side Drawer**
  - Requires separate responsive layout rules and split view management.
- **Approach 3: Inline Card Editing**
  - Cramped inside column width; insufficient space for timestamps and status dropdowns.
