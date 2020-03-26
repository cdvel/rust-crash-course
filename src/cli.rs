use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("Commands: {:?}", command);
    let status = "100%";
    let name = "bran";
    if command == "hello" {
        println!("Hi {}, how r u?", name);
    } else if command == "status" {
        println!("Status {}", status);
    }else{
        println!("Not valid");
    }
}