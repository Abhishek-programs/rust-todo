use std::env;

pub fn run() {
    println!("Hello, from the cli.rs file");

    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    // let name = args[2].clone();
    // let age = args[3].clone();

    println!("Arguments: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hello, world!");
    } else if command == "help" {
        println!("Available commands: hello, help");
    } else {
        println!("Unknown command");
    }
}
