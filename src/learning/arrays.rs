use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[4]);

    // reassign value
    numbers[4] = 20;
    println!("Reassigned value: {}", numbers[4]);
    println!("{:?}", numbers);

    // get length of array
    println!("Length: {}", numbers.len());

    // stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[..2];
    println!("Slice: {:?}", slice);
}
