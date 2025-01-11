use chrono::{DateTime, Local};
use std::fs;
use std::io::{self, Write};
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Todo {
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Local>, 
    description: String,
    completed: bool,
}

impl Todo {
    fn new(description: String) -> Self {
        Todo {
            created_at: Local::now(), 
            description,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    } 
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn save_tasks(todos: &Vec<Todo>) {
    match File::create("todos.json") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(serde_json::to_string(todos).unwrap().as_bytes()) {
                eprintln!("Failed to write to file: {}", e);
            } else {
                println!("Tasks saved!");
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}

fn load_tasks() -> Vec<Todo> {
    match fs::read_to_string("todos.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| {
            println!("Failed to parse tasks. Starting with an empty list.");
            Vec::new()
        }),
        Err(_) => {
            println!("No previous tasks found. Starting fresh.");
            Vec::new()
        }
    }
}

fn main() {
    let mut todos: Vec<Todo> = load_tasks();

    loop {
        println!("1. Add Task\n2. List Tasks\n3. Mark a Task\n4. Save Tasks as JSON\n5. Exit");
        let choice = get_user_input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let description = get_user_input("Enter task description: ");
                let task = Todo::new(description);
                todos.push(task);
                println!("Task added!");
            }

            "2" => {
                if todos.is_empty() {
                    println!("No tasks are available!");
                } else {
                    for (index, todo) in todos.iter().enumerate() {
                        println!(
                            "{}. [{}] {} : {}",
                            index + 1,
                            if todo.completed { "X" } else { " " },
                            todo.description,
                            todo.created_at.format("%Y-%m-%d %H:%M:%S")
                        );
                    }
                }
            }

            "3" => {
                if todos.is_empty() {
                    println!("No tasks are available!");
                } else {
                    let task_number = get_user_input("Enter task number to mark as completed: ");
                    if let Ok(index) = task_number.parse::<usize>() {
                        if index > 0 && index <= todos.len() {
                            todos[index - 1].mark_completed();
                            println!("Task marked successfully!");
                        } else {
                            println!("Invalid Task Number! Please choose a number between 1 and {}.", todos.len());
                        }
                    } else {
                        println!("Invalid Input! Please enter a number.");
                    }
                }
            }

            "4" => {
                if todos.is_empty() {
                    println!("No tasks are available!");
                } else {
                    save_tasks(&todos);
                }
            }

            "5" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice! Try again."),
        }
    }
}
