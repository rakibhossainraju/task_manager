# Drag & Drop Task Reordering & Column Movement Design Specification

## Overview & Intent
Implement fluid, intuitive drag-and-drop capability for Kanban task cards. Users can drag any task card and drop it into any column at a specific target position, updating its column status and order immediately.

## Goals
1. **Domain Logic for Reordering**: Add `move_task(&mut self, task_id: Uuid, target_group_id: Uuid, target_index: usize) -> bool` to `BoardState`.
   - Handles intra-column reordering (e.g. moving a task from position 0 to 2 in "Todo").
   - Handles inter-column movement (e.g. moving a task from "Todo" to "Progress" at position 1).
   - Updates `task.updated_at` on movement.
   - Comprehensive unit tests covering inter- and intra-column moves, bounds clamping, and nonexistent IDs.
2. **Native HTML5 Drag & Drop Interactions**:
   - `TaskCard` is marked `draggable: "true"`.
   - `ondragstart`: Stores dragged `task_id` in a root drag signal `Signal<Option<Uuid>>`. Adds visual dragging state (`opacity-50`).
   - `ondragover` / `ondrop` on cards: Calculates drop index and invokes `board.write().move_task(...)`.
   - `ondragover` / `ondrop` on columns (empty areas / container): Drops the task at the end of that column.
   - `ondragend`: Cleans up drag signal and reset highlights.
3. **Visual Feedback**:
   - Subtle drop indicator (e.g. emerald accent border or highlight line) indicating where the card will land.

## Non-Goals
- Dragging columns to reorder entire columns (out of scope for this phase).
- Multi-card drag selection.

## Approaches Considered
- **Approach 1: Native HTML5 Drag & Drop with Dioxus State Context (Selected)**
  - Uses standard Dioxus HTML drag events (`ondragstart`, `ondragover`, `ondrop`, `ondragend`) backed by a shared `Signal<Option<Uuid>>` context.
  - *Tradeoffs*: Lightweight, zero external crates, accelerated by host webview/browser, perfectly cross-platform for Dioxus web & desktop. Selected by user.
- **Approach 2: Manual Pointer / Coordinate Tracking**
  - Tracking pointer movements and calculating layout client rects.
  - *Tradeoffs*: Highly complex, prone to coordinate desync on desktop webviews, heavy CPU overhead. Rejected.

## Component & Model Structure
- `src/models/board.rs`: Add `move_task` method with unit test suite.
- `src/components/task_manager.rs`: Provide `use_context_provider(|| Signal::new(None::<Uuid>))` as `DragTaskContext`.
- `src/components/tasks/task_card.rs`: Add drag handlers and drop target event listeners.
- `src/components/tasks/task_column.rs`: Add column container drop listener for empty spaces.
