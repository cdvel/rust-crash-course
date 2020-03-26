//Functions used to store blocks of code to reuse

pub fn run(){
    greetings("Hello", "Jane");    
    //Bind function values to variable
    let get_sum = add(5, 5);
    println!("Sum : {}", get_sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,3 ));
}


fn greetings(greet: &str, name: &str){
    println!("{} {} nice 2 meet", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}