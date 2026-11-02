# Modal Task Creation & Toolbar Cleanup Design Specification

## Overview & Intent
Replace the per-column inline creation forms and footer "+ Add Item" buttons with a single, prominent "Add Task" button in the top toolbar. Clicking it opens a root-level modal dialog to create a task with Title, Description, Priority, and Status (target column). Remove non-functional "Discard" and "Save" toolbar buttons to clean up the UI.

## Goals
1. **Clean Top Toolbar**: Remove obsolete "Discard" and "Save" buttons from `Filter`. Add a styled "Add Task" button (with a `+` icon and green accent or matching theme).
2. **Root-Level Task Creation Modal**: Render a modal overlay in `TaskManager` that sits above all board elements.
3. **Form Fields**:
   - Title: Required, trimmed, validated.
   - Description: Optional text input.
   - Priority: Dropdown/selector with Low, Medium, High, Urgent.
   - Status: Dynamic dropdown populated with existing columns from `BoardState`, defaulting to the first column (e.g. "Todo").
4. **Keyboard & Backdrop Interaction**:
   - <kbd>Esc</kbd> or clicking the backdrop cancels and closes the modal.
   - <kbd>Enter</kbd> in inputs submits the form.
5. **Column Simplification**: Remove inline creation forms and footer "+ Add Item" buttons from `TaskColumn`.

## Non-Goals
- Column deletion / renaming modals.
- Drag and drop tasks.

## Approaches Considered
- **Approach 1: Root-Level Overlay in `TaskManager` (Selected)**
  - Toggle a modal component placed at the root level of `TaskManager` via a shared signal (`use_signal` passed down or context).
  - *Tradeoffs*: Prevents CSS clipping or stacking context issues across columns/overflow scrollbars. Selected by user for cleaner separation and reliability.
- **Approach 2: Scoped Modal in `Filter`**
  - Keep the dialog inside the `Filter` toolbar component.
  - *Tradeoffs*: Harder to manage z-index and fixed overlay across full-height Kanban board layout. Rejected.

## Component & State Details
- `CreateTaskModal`: A new component in `src/components/modal/create_task.rs` (or `src/components/tasks/create_task_modal.rs`).
- `Filter`: Replaces "Discard" and "Save" buttons with "+ Add Task" button which toggles modal open state.
- `TaskColumn`: Stripped of `is_adding`, `new_title`, `new_desc`, and footer button, rendering strictly the header and task cards.
- Tests: Unit tests for state updates remain green, and all existing board tests pass.
