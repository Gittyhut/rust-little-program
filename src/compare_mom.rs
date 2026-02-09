use std::cmp::Ordering;

use std::io;

fn main() {
  println!("Compare your age with my mom's age");

  println!("How old are you?");

  let mom_age = 39;

  let mut age = String::new();

  io::Stdin()
    .read_line(&mut age)
    .expect("Failed to read line.");
  
  let age: i64 = age.trim().parse().expect("How old are you?");

  match age.cmp(&mom_age) {
    Ordering::Less => println!("You're younger than my mom!"),
    Ordering::Greater => println!("You're older than my mom!"),
    Ordering::Equal => println!("You're the same age as my mom!"),
  }
}
