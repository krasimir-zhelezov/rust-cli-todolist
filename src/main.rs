use std::{collections::HashMap, io::{self, Read}};
use uuid::Uuid;

#[derive(Debug)]
struct Task {
    id: String,
    name: String,
    description: String,
    done: bool
}

struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self) {
        println!("Name: ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        println!("Description: ");

        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");

        self.tasks.push(Task {
            id: Uuid::new_v4().to_string(),
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            done: false
        });
    }

    fn view_tasks(&self) {
        println!("{:#?}", self.tasks);
    }

    fn complete_task(&mut self, id: String) {
        for task in &mut self.tasks {
            if task.id == id {
                task.done = true;
            }
        }
    }

    fn delete_task(&mut self, id: String) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].id == id {
                self.tasks.remove(i);
                break;
            }
        }
    }
}

fn main() {
    // let mut tasks: Vec<Task> = Vec::new();
    let mut task_manager = TaskManager::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let input: Vec<&str> = input.split(" ").collect();

        match input[0] {
            "exit" => break,
            "add" => task_manager.add_task(),
            "view" => task_manager.view_tasks(),
            "delete" => task_manager.delete_task(input[1].to_string()),
            "complete" => task_manager.complete_task(input[1].to_string()),
            _ => println!("Uknown command"),
        }
    }
}