use std::cmp::Ordering;
#[allow(dead_code)]
use std::io::stdin;

use colored::Colorize;
use rand::Rng;

pub fn chapter2() {
    println!("Guessing Game");

    let random_num = rand::thread_rng().gen_range(1, 101);
    println!("The random number {random_num}");

    loop {
        println!("Enter your guess");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("{}", "Too Small".blue()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", "Congratulations!!".green());
                break;
            }
        }
    }
}
