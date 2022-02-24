pub fn run(){
    let name = "Aiman";
    let mut age = 24;
   
    println!("{} is {}",name, age);
    
    age = 32; // Age must be mutable. Use mut

    println!("{} is {}",name, age);

    const ID: i8 = 001; //i8 denotes int_8bit
    println!("{}", ID);

    let (other_name, other_age) = ("Bob", 21);

    println!("{} is {}", other_name, other_age);  
;}
