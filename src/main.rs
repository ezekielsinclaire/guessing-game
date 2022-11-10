use std::io;

fn main() {
    print!("{}", option());
}

fn option() -> i32 {
    let mut input = String::new();
    println!("Hello, choose an option: ");
    println!("1: Guess the word");
    println!("2: Guess the integer");
    io::stdin().read_line(&mut input).expect("error: failure to read option");
    let option: i32 = input.trim().parse().expect("Input not an integer");
    return option;
}