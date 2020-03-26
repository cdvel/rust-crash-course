// Conditionals - Used to check condition of something and act

pub fn run(){
    let age: u8 = 10;
    let check_id: bool = true;
    let knows_person_of_age = true;

    //If/Else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender, drink?");
    }else if age < 21 && check_id {
        println!("Bartender, out!");
    }else {
        println!("Bartender, ID?");
    }

    //Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}