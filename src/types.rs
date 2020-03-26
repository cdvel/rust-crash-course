/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, u128 (bits from memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means tahat ir must know the types of 
// all vartiables at compile time, however, the compiles can usually infer what type 
// we want to us based on the value and how we use it

pub fn run(){
    //Default is "i32"
    let x = 1;

    // Defaultt is f64
    let y=2.4;

    // Add explicit type
    let z: i64 = 432432432443;

    println!(" x,y,z = {} {} {}", x, y, z);

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean 
    let is_active = true;

    //Get boolean from expression
    let is_greater: bool = 10 > 11;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}