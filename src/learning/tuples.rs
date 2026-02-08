pub fn run() {
    let person: (&str, &str, i8) = ("John", "Canada", 30);
    println!(
        "{} is from {} and is {} years old",
        person.0, person.1, person.2
    );
}
