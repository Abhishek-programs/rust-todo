pub fn run() {
    let name = "John";
    let mut age = 30;

    println!("Age: {}", age);
    age = 31;

    println!("Name: {}", name);
    println!("Age: {}", age);

    let (my_name, my_age) = ("John", 30);
    println!("Name: {}", my_name);
    println!("Age: {}", my_age);
}
