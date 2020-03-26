pub fn run() {
    //Print to console
    println!("!Hello workl");

    // Basic formatting
    println!("{} is from {}", "Cd","Co");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}", 
        "cdv", "co", "play"
    );

    // Named arguments
    println!(
        "{name} likes to {activity}", 
        name="Cdv",
        activity="run",
     );

     //Placeholder traits
     println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait (tuple)
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

}