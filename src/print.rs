pub fn run(){
    // Print to consoles
    println!("Hello from print.rs");

    println!("Number is {}", 1);

    println!("{} works as a {}", "Aiman","Software Eng");

    println!("{0} works as a {1}. {0} is learning {2}", "Aiman", "Software Eng", "Rust"); //Positions indexed by postion.

    println!("{name} likes to play {sport}", name = "Aiman", sport = "Badminton");

    println!("Binary: {:b}", 10);
}