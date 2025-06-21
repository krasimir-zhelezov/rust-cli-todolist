use std::{collections::HashMap, io::{self, Read}};

#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    done: bool
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "exit" => break,
            "add" => tasks.push(add_task()),
            "view" => println!("{:#?}", tasks),
            "delete" => delete_task(tasks.as_mut()),
            "complete" => complete_task(tasks.as_mut()),
            _ => println!("Uknown command"),
        }
    }
}

fn add_task() -> Task {
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

    Task {
        name: name.to_string(),
        description: description.to_string(),
        done: false
    }
}

// fn view_tasks(tasks: &mut Vec<HashMap<String, String>>) {
//     for (i, task) in tasks.iter().enumerate() {
//         println!("----- TASK #{} -----", i);
//         println!("name: {}", task.get("name").unwrap());
//         println!("description: {}", task.get("desc").unwrap());
//         println!("done: {}", task.get("done").unwrap());
//     }
// }

fn delete_task(tasks: &mut Vec<Task>) {
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");

    let id: usize = id.trim().parse().expect("Not a valid usize");

    tasks.remove(id);
}

fn complete_task(tasks: &mut[Task]) {
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");

    let id: usize = id.trim().parse().expect("Not a valid usize");
    tasks[id].done = true;
}