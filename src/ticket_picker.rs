use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main() {
  println!("Ticket, ticket!");

  let winner_number = rand::thread_rng().gen_range(1..=100);

  let mut random = rand::thread_rng().gen_range(1..=300);
  
  println!("your number is:{random}");

  println!("the winner's number is: {winner_number}");
    
   match random.cmp(&winner_number) {
     Ordering::Less => println!("You failed!"),
     Ordering::Greater => println!("You failed!"),
     Ordering::Equal => println!("You won the prize!"),
   }
}
