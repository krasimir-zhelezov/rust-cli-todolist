use std::{collections::HashMap, io::{self, Read}};

fn main() {
    let mut tasks: Vec<HashMap<String, String>> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        } else if input == "add" {
            add_task(tasks.as_mut());
        } else if input == "view" {
            view_tasks(tasks.as_mut());
        } else if input == "delete" {
            delete_task(tasks.as_mut());
        } else if input == "complete" {
            complete_task(tasks.as_mut());
        }
    }
}

fn add_task(tasks: &mut Vec<HashMap<String, String>>) {
    println!("Name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Description: ");

    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    let description = description.trim();

    let mut task = HashMap::new();

    task.insert("name".to_string(), name.to_string());
    task.insert("desc".to_string(), description.to_string());
    task.insert("done".to_string(), false.to_string());

    tasks.push(task);
}

fn view_tasks(tasks: &mut Vec<HashMap<String, String>>) {
    for (i, task) in tasks.iter().enumerate() {
        println!("----- TASK #{} -----", i);
        println!("name: {}", task.get("name").unwrap());
        println!("description: {}", task.get("desc").unwrap());
        println!("done: {}", task.get("done").unwrap());
    }
}

fn delete_task(tasks: &mut Vec<HashMap<String, String>>) {
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");

    let id: usize = id.trim().parse().expect("Not a valid usize");

    tasks.remove(id);
}

fn complete_task(tasks: &mut Vec<HashMap<String, String>>) {
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");

    let id: usize = id.trim().parse().expect("Not a valid usize");
    tasks[id].insert("done".to_string(), true.to_string());
}