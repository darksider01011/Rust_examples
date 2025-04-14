use std::io;
use rand::Rng;
use std::cmp::Ordering;

// var type function
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// main function
fn main() {
        println!("Guess the number!");
        println!("");

        // generate random number 
        let secret_number = rand::thread_rng().gen_range(1..=100);
        //println!("The secret number is: {secret_number}");
        println!("");

        // infinity loop
        loop {
        println!("Please input your guess.");

        // define type guess variable 
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // convert string type to integer 32bit
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match statement
        match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                        println!("You Win!");
                        // break the tools if user guess true
                        break;
                        }}
        }
