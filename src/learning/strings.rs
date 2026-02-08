pub fn run() {
    let mut hello = String::from("Hello ");
    // let world = String::from("World");
    // let hello_world = hello + &world;

    hello.push_str("World");

    // contains
    println!("contains 'World': {}", hello.contains("World"));

    // replace
    println!("replace 'World': {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
    println!("Length: {}", hello.len());
}
