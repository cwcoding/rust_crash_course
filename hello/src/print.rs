pub fn run() {
    println!("===== PRINTING =====");
    
    // Print to console
    println!("Hello from print.rs !");

    // Positional arguments
    println!("{0} is from {1} and {0} and likes to {2}", "Cory", "CT", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Cory", activity="the drums");

    // Placeholder traits
    println!("binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits (tuple used here)
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}