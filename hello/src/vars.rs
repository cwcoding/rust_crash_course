// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    println!("===== VARIABLES =====");
    
    let name = "Banik";
    let mut age = 35;

    println!("His name is {} and he is {}", name, age);
    age = 38;
    println!("His name is {} and he is {}", name, age);

    // Define constant
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars

    let (my_name, my_age ) = ("Banik", 32);
    println!("{} is {}", my_name, my_age);
}