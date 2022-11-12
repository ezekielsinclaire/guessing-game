#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;
use rand::prelude::*;

fn main() {
    run();
}

fn word_game() {
    let msg = "error: failure to read word";
    let words: [&str; 3] = ["list", "of", "words"];
    let mut rng = rand::thread_rng();
    let word: String = words[rng.gen_range(0..3)].trim().parse().expect(msg);

    println!("Enter one of the choice words from the list below: ");
    println!("{:?}", words);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(msg);
    let option: String = input.trim().parse().expect(msg);

    // check if word list contains user input
    let s_slice: &str = &option[..];
    if !(words.contains(&s_slice)) {
        println!("error: input must be one of the words from the list");
        return;
    }

    // compare user input to random word
    if option == word {
        print!("You win!")
    } else {
        print!("You lose, the correct word was '{}'", word)
    }

}

fn number_game() {
    let msg = "error: failure to read number";
    let mut rng = rand::thread_rng();
    let number: i32 = rng.gen_range(1..=5);

    println!("Enter an integer between 1 - 5 (including): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(msg);
    let input: i32 = input.trim().parse().expect(msg);

    // check if 1 > user input > 5
    if input < 1 || input > 5 {
        println!("error: input must be an integer between 1 - 5 (including)");
        return;
    }

    // compare user input to random number
    if input == number {
        print!("You win!")
    } else {
        print!("You lose, the correct number was {}", number)
    }
}

fn run() {
    let mut input = String::new();
    println!("Hello, choose an option: ");
    println!("1: Guess the word");
    println!("2: Guess the integer");
    io::stdin().read_line(&mut input).expect("error: failure to read option");
    let option: i32 = input.trim().parse().expect("Input not an integer");


    if option == 1 {
        word_game();
    } else if option == 2 {
        number_game();
    } else {
        println!("error: option must either be 1 or 2");
        return;
    }
}
