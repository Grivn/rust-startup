use std::io;

fn main() {
    println!("Guessing the number!");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Gailed to read line");
    
    println!("You guessed: {guess}");
}
