use std::io;

fn main() {
    println!("guess the number!");
    
    println!("Please enter your number");
    
    let mut guess = String::new();
    
    io::stdin();
    .read_line(mut& guess);
    .expect("failed to read line");
    
    println!("your guess is: {guess}");
}