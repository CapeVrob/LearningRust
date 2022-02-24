pub fn run(){
    let a:i32 = 72;
    let b = 72; //Will be inferred by the compiler as an i32
    let c = 2.1; //Will be inferred by the compiler as a f64

    println!("{},{},{}", a,b,c);
    
    //Finding max size
    println!("{}", std::i32::MAX);
    //println!("{}", std::f64::MAX);

    let what:bool = false;

    let is_greater = 3 > 2;

    let single_char = 'a';

    let emoji = '\u{1f600}';

    //Can print everything by putting :? in the curly braces
    println!("{:?}", (a,b,c,std::i32::MAX, what, is_greater, single_char,emoji));
}