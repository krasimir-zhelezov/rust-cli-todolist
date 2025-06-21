# Rust Task Manager

A simple command-line task manager written in Rust.

## Features

- Add new tasks with name and description
- View all tasks with their status
- Mark tasks as complete
- Delete tasks
- Interactive command-line interface

## Commands

- `add` - Add a new task
- `view` - View all tasks
- `delete` - Delete a task (prompts for task ID)
- `complete` - Mark a task as complete (prompts for task ID)
- `exit` - Exit the application

## How to Run

1. Make sure you have Rust installed (install via [rustup](https://rustup.rs/))
2. Clone this repository
3. Run the application:

```sh
cargo run
```

## Usage Example
```sh
> add
Name: Buy groceries
Description: Milk, eggs, bread

> add
Name: Finish project
Description: Complete the Rust task manager

> view
----- TASK #0 -----
name: Buy groceries
description: Milk, eggs, bread
done: false
----- TASK #1 -----
name: Finish project
description: Complete the Rust task manager
done: false

> complete
1

> view
----- TASK #0 -----
name: Buy groceries
description: Milk, eggs, bread
done: false
----- TASK #1 -----
name: Finish project
description: Complete the Rust task manager
done: true

> exit
```

## Data Structure
Tasks are stored as a ```Vec<HashMap<String, String>>``` where each task has:

- name - The task name

- desc - The task description

- done - Completion status (true/false)

## Limitations
- No persistent storage (tasks are lost when program exits)

- Basic error handling

- Simple command-line interface


## LICENSE
This project is licensed under the MIT License (Non-Commercial).