pub fn run() {
    println!("Hello from the function.rs file");

    greet("Hello", "World");
    let get_sum = add(1, 2);
    println!("Sum: {}", get_sum);

    // closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum: {}", add_nums(3, 3));
}

fn greet(greeting: &str, name: &str) {
    println!("{} {}", greeting, name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
