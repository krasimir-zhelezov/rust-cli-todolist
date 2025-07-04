use std::{fs, io::{self}};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: String,
    name: String,
    description: String,
    done: bool,
    timestamp: DateTime<Utc>
}

struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    fn new() -> Self {
        let json = match fs::read_to_string("tasks.json") {
            Ok(content) => content,
            Err(_) => String::from("[]")
        };

        TaskManager { tasks: serde_json::from_str(&json).unwrap() }
    }

    fn add_task(&mut self, name: String, description: String) {
        self.tasks.push(Task {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            done: false,
            timestamp: Utc::now()
        });

        self.save_tasks();
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
        self.save_tasks();
    }

    fn delete_task(&mut self, id: String) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].id == id {
                self.tasks.remove(i);
                break;
            }
        }
        self.save_tasks();
    }

    fn save_tasks(&self) {
        let json = serde_json::to_string_pretty(&self.tasks).unwrap();
        fs::write("tasks.json", json).expect("Unable to write file");
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
            "add" => task_manager.add_task(input[1].to_string(), input[2].to_string()),
            "view" => task_manager.view_tasks(),
            "delete" => task_manager.delete_task(input[1].to_string()),
            "complete" => task_manager.complete_task(input[1].to_string()),
            _ => println!("Unknown command"),
        }
    }
}