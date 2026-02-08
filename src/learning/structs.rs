struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        }
    }

    fn full_name(&self) -> String {
        let full_name = format!("{} {}", self.first_name, self.last_name);
        full_name
    }

    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    fn name_to_tuple(&self) -> (String, String, u8) {
        (self.first_name.clone(), self.last_name.clone(), self.age)
    }
}

pub fn run() {
    let mut person = Person::new("John", "Doe", 30);
    println!(
        "Person: {} {} {}",
        person.first_name, person.last_name, person.age
    );

    println!("Person full name: {}", person.full_name());

    person.set_last_name("Williams");
    println!("Person full name: {}", person.full_name());

    let name_tuple = person.name_to_tuple();
    println!("Person name tuple: {:?}", name_tuple);

    // let mut person: (&str, &str, i8) = ("John", "Canada", 30);
    // println!(
    //     "{} is from {} and is {} years old",
    //     person.0, person.1, person.2
    // );

    // struct Color {
    //     red: u8,
    //     green: u8,
    //     blue: u8,
    // }

    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // println!("Color: {} {} {}", c.red, c.green, c.blue);
    // c.red = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // c.green = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // tuple struct
    // struct Color(u8, u8, u8);
    // let mut c = Color(255, 0, 0);
    // println!("Color: {} {} {}", c.0, c.1, c.2);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);
    // c.1 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);
    // c.2 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    // // Printing tuple struct with debug formatter
    // println!("Color: {:?}", (c.0, c.1, c.2));
}
