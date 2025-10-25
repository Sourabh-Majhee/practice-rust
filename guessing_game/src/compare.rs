use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main(){
    // --snip--

    let mut guess = String::new();

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}")

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}