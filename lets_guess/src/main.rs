use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome");

    println!("Please Provide your input here!!");

    //? This will make it mutable as data types in rust are immutable
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Number is : {}", secret_number);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read-lines");

        // This is type of shadowing where you keep the var name same but change its type
        // let guess: i32 = guess.trim().parse().expect("Error while parsing"); //* This will terminate the loop so other way is to not terminate is below */
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed number : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Less".red()),
            Ordering::Equal => {
                println!("{}", "Equal".green());
                break;
            }
            Ordering::Greater => println!("{}", "Greater"),
        }
    }
}
