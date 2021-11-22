// tuples group together values of different types
// Max 12 elements (?)

pub fn run() {
    println!("===== TUPLES =====");
    
    let person: (&str, &str, i8) = ("Banik", "CAD", 80);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}