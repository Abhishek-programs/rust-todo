pub fn run() {
    println!("Hello, from the print.rs file");

    println!("{}", 1);

    println!("{0} {1} {0}", "Hello", "World");

    println!("{:?}", (12, true, "Hello"));

    println!("{name} {age}", name = "John", age = 30);
}
