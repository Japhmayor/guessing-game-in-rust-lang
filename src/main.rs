// import an input/output module from the standard library
// used to accept input from users
use std::io;
// import crate for getting random number functionality
use rand::Rng;
// for comparing and ordering
use std::cmp::Ordering;

// the main function
fn main() {
    println!("Welcome to the guessing game");
    println!("Please enter your guess");

    // generate a random number from 1 - 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // print out the secret number
    println!("The secret number is {}", secret_number);
    // create a loop
    loop {
        // create a variable that holds the user guessed number as an empty string
        let mut guess = String::new();

        //calls a function that makes it possible to accept user input
        io::stdin()
            // Method to get input from the user. read_line take whatever the user
            // types into standard input [stdin()] and place that into a string.
            // the & incicates that this argument is a reference, which gives you a
            // way to let multiple parts of your code access one
            // piece of data without needing to copy that data into memory multiple times.
            .read_line(&mut guess)
            // This line is for error handling, it is compulsory
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type in your number");

        // print out the users guess
        println!("The number you guessed is {}", guess);

        // match guess to secret number and compare them
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
