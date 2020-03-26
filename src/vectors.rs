// Vectors - List where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //Re-asign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    numbers.pop();

    println!("{:?}", numbers);

    //Get single val
    println!("Single value: {}", numbers[0]);

    // Get Vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;        
    }

    println!("Numbers Vec: {:?}", numbers);
}
