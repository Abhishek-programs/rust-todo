// Simple CLI Todo App
// Run with: cargo run -- <command> [args]
// Examples:
//   cargo run -- list
//   cargo run -- add "Learn Rust"
//   cargo run -- done 1
//   cargo run -- help
//
// Todos are saved in a file (.todos) so they persist between runs.

use std::env;
use std::fs;
use std::io;

const TODO_FILE: &str = ".todos";

/// Load todos from file. Returns empty vec if file doesn't exist yet.
fn load_todos() -> Vec<String> {
    match fs::read_to_string(TODO_FILE) {
        Ok(contents) => contents
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Save todos to file (one per line).
fn save_todos(todos: &[String]) -> io::Result<()> {
    let contents = todos.join("\n");
    fs::write(TODO_FILE, contents)
}

pub fn run() {
    // Load existing todos from file (or start with empty list)
    let mut todos = load_todos();

    // env::args() gives us what the user typed after "cargo run --"
    // args[0] is the program name, args[1] is first command, etc.
    let args: Vec<String> = env::args().collect();

    // Need at least one argument (the command)
    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "list" => {
            if todos.is_empty() {
                println!("Your list is empty. Add something with: cargo run -- add \"Your task\"");
            } else {
                println!("Your todos:");
                for (i, todo) in todos.iter().enumerate() {
                    println!("  {}. {}", i + 1, todo);
                }
            }
        }
        "add" => {
            if args.len() < 3 {
                println!("Usage: cargo run -- add \"Your task here\"");
                return;
            }
            // Join everything after "add" in case the task has spaces
            let task = args[2..].join(" ");
            todos.push(task.clone());
            if save_todos(&todos).is_ok() {
                println!("Added: {}", task);
            } else {
                println!("Added: {} (could not save to file)", task);
            }
        }
        "done" | "remove" => {
            if args.len() < 3 {
                println!("Usage: cargo run -- done <number>");
                return;
            }
            let num: usize = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Please enter a valid number (e.g. 1, 2, 3)");
                    return;
                }
            };
            if num == 0 || num > todos.len() {
                println!("Invalid number. You have {} items.", todos.len());
                return;
            }
            let removed = todos.remove(num - 1);
            if save_todos(&todos).is_ok() {
                println!("Done: {}", removed);
            } else {
                println!("Done: {} (could not save to file)", removed);
            }
        }
        "help" | "-h" | "--help" => print_help(),
        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("Todo App - Simple CLI");
    println!();
    println!("Commands:");
    println!("  list              Show all todos");
    println!("  add \"task\"        Add a new todo");
    println!("  done <number>     Mark todo as done (remove by number)");
    println!("  remove <number>   Same as done");
    println!("  help              Show this message");
    println!();
    println!("Examples:");
    println!("  cargo run -- list");
    println!("  cargo run -- add \"Buy groceries\"");
    println!("  cargo run -- done 1");
}
