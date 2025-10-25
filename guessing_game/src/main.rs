use std::io;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let mut new_number = String::new();

    println!("Add another number");

    io::stdin().read_line(&mut new_number).expect("Failed to read line");


    println!("You guessed: {guess} + {new_number}");
}