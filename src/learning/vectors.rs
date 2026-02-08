use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[4]);

    // reassign value
    numbers[4] = 20;
    println!("Reassigned value: {}", numbers[4]);

    // add to vector
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);

    // get length of vector
    println!("Length: {}", numbers.len());

    // stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[..2];
    println!("Slice: {:?}", slice);

    // loop through vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop through vector and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
        // 0 if even, 1 if odd
        // *x = if *x % 2 == 0 { 0 } else { 1 };
    }
    println!("Vector: {:?}", numbers);

    // pop from vector
    numbers.pop();
    println!("Vector: {:?}", numbers);

    // remove from vector
    numbers.remove(0);
    println!("Vector: {:?}", numbers);
}
