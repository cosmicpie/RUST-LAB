use std::cmp::Ordering;
use std::io;
use rand::prelude::*;


fn main() {
    println!("Hello, kaki jiiiii");

    let secret_num = rand::rng().random_range(1..=100);
    loop{
    println!("apna number guse karo bhai:-");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("faild to read line");
    println!("you guess: {}", guess);

    let guess_num: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess_num.cmp(&secret_num) {
        Ordering::Equal => { println!("yeee you win"); break; }
        Ordering::Less => println!("your guess is too small"),
        Ordering::Greater => println!("your guess is tooo big"),
    }
  }
}
