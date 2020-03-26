//Structs - Used to create custom data types

//Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//Tuple struct
// struct Color(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str){
        self.last_name =  last.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    // let mut c = Color{
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut c = Color(255, 255, 255);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let p = Person::new("Jon", "Snow");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    let mut p2 = Person::new("Mary", "Doe");
    p2.set_last_name("Williams");
    println!("Person {}", p2.full_name());
    println!("Person {:?}", p2.to_tuple());


}