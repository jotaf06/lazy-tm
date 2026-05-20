# lazy-tm

A terminal-based task manager built with Rust and [ratatui](https://github.com/ratatui-org/ratatui).

## Features

- Multiple named task lists, each saved independently
- Add, edit, and delete tasks with a title and description
- Toggle tasks as done / not done
- Per-task stopwatch — track time spent on each task
- Timers pause automatically when a task is marked complete
- Two-panel layout to browse lists and tasks side by side

## How to use

### Home screen

When you launch the app you land on the **Home screen**, which lists all your saved task lists.

- Navigate with `j` / `k`
- Press `Enter` to open a list
- Press `a` to create a new list (type the name, confirm with `Enter`)
- Press `D` to delete the selected list (requires confirmation)
- Press `M` to switch to two-panel mode
- Press `q` to quit

### Task list

After opening a list you enter the **Task list** view.

- Navigate tasks with `j` / `k`
- Press `Space` to toggle a task done / not done
- Press `a` to add a new task
- Press `e` to edit the selected task
- Press `T` to attach a stopwatch to the selected task
- Press `S` to start or stop that stopwatch
- Press `D` to delete the selected task (requires confirmation)
- Press `C` to clear all tasks (requires confirmation)
- Press `q` or `Esc` to save and go back to the Home screen

### Two-panel mode

Press `M` from either screen to enter **two-panel mode**, where the list browser sits on the left and the task list on the right.

- Switch focus between panels with `1` (left) and `2` / `Enter` (right)
- All normal keybindings apply per panel
- Press `M` again to return to single-panel mode

### Editing mode

Triggered by `a` (add) or `e` (edit):

| Key     | Action                             |
|---------|------------------------------------|
| `Tab`   | Switch between Title / Description |
| `Enter` | Save                               |
| `Esc`   | Cancel                             |

## Keybindings summary

### Home screen

| Key         | Action              |
|-------------|---------------------|
| `j` / Down  | Next list           |
| `k` / Up    | Previous list       |
| `Enter`     | Open list           |
| `a`         | New list            |
| `D`         | Delete list         |
| `M`         | Toggle two-panel    |
| `q`         | Quit                |

### Task list

| Key         | Action              |
|-------------|---------------------|
| `j` / Down  | Next task           |
| `k` / Up    | Previous task       |
| `Space`     | Toggle done         |
| `a`         | Add task            |
| `e`         | Edit task           |
| `T`         | Create stopwatch    |
| `S`         | Start / stop timer  |
| `D`         | Delete task         |
| `C`         | Clear all tasks     |
| `M`         | Toggle two-panel    |
| `q` / `Esc` | Save & go back      |

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

## Running

```bash
cargo run
```

Lists are saved to the `lists/` directory in the current working directory. If a legacy `tasks.json` file is found on first launch it is automatically migrated to `lists/Default.json`.

## Tech stack

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI rendering
- [serde / serde_json](https://serde.rs/) — task persistence
- [color-eyre](https://github.com/eyre-rs/color-eyre) — error reporting

---

## Update notes (20/05/2026)

### Multiple named task lists + Home screen

The app now supports any number of named task lists instead of a single `tasks.json` file.

- A **Home screen** is shown on startup, listing all available lists.
- Each list is stored as its own file under `lists/<name>.json`.
- Lists can be created, opened, and deleted from the Home screen.
- If a `tasks.json` file exists from a previous version it is automatically migrated to `lists/Default.json` on first launch.

### Two-panel layout

Press `M` to split the screen: the list browser on the left and the task list on the right. Navigating lists on the left immediately loads their tasks on the right. Switch focus with `1` and `2`. Press `M` again to go back to single-panel mode.

### Per-task stopwatch

Press `T` on a selected task to attach a stopwatch to it, then `S` to start or pause it. The elapsed time is shown next to the task. The timer stops automatically when the task is marked done.

### Edit task

Press `e` to open the selected task in the editing popup. Use `Tab` to switch between the Title and Description fields. Press `Enter` to save or `Esc` to cancel.
