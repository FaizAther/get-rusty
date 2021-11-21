use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use colored::*;

fn main() {
    println!("Hello, world!");
    let mut rng = thread_rng();
    let secret_number: u32 = rng.gen_range(0..101);
    println!("{}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess:u32 = guess.trim().parse().expect("Type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too big".blue()),
            Ordering::Equal => {
                println!("{}", "Too equal".green());
                break;
            },
        }
    }
}
