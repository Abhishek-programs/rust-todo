pub fn run() {
    let age = 22;

    let is_adult = if age >= 21 { true } else { false };

    if is_adult {
        println!("You are an adult");
    } else {
        println!("You are not an adult");
    }
}
