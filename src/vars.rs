// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is block-scoped language

pub fn run(){
    let name = "C";
    let mut age = 10;
    println!("I am {} and my age is {}", name, age);
    age = 38;
    println!("I am {} and my age is {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Cd", 39);
    println!("{} is {}", my_name, my_age);
}