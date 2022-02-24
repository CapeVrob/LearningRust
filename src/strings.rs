//str and String is different.
//str is immutable
//String is mutable, can grow and be modified.

pub fn run(){
    let mut test_string = String::from("Ben Purdy ");

    println!("{}", test_string.len());

    test_string.push('\u{1f600}');

    println!("{}", test_string);

    test_string.pop();
    println!("{}", test_string);

}