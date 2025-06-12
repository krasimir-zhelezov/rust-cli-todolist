use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        } else if input == "add" {
            add_task();
        }
    }
}

fn add_task() {
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

    println!("Task: {}", name);
}