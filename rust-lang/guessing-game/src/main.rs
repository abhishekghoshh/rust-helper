use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    // guess_one_number();
    guess_until_random();
}
// it will work for one time only,
// but we need a loop to ask for the input until it is the random number
pub fn guess_one_number(){
    // it will generate a number from 1 to 100
    let random_number = rand::thread_rng()
        .gen_range(1..=100);
    println!("guessing game");
    println!("Guess a number from 1 to 100 :");

    let mut guessed = String::new();

    io::stdin()
        .read_line(&mut guessed)
        .expect("Failed to take any input");


    println!("random number is {random_number}");
    println!("your guessed number is {guessed}");

    let guess_number:i32 = guessed.trim()
        .parse()
        .expect("Please type a number");

    match guess_number.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

pub fn guess_until_random(){
    let random_number = rand::thread_rng()
        .gen_range(1..=100);
    loop{
        println!("welcome to guessing game");
        println!("please enter a number : ");
        let mut guessed = String::new();
        io::stdin()
            .read_line(&mut guessed)
            .expect("Error while taking the input");


        let guessed_number:i32 = match guessed.trim().parse() {
            Ok(num)=> num,
            Err(_)=> {
                println!("please enter a valid number");
                continue;
            }
        };

        // it will clear the screen
        std::process::Command::new("clear").status().unwrap();

        match guessed_number.cmp(&random_number) {
            Ordering::Less => println!("your guessed number is smaller"),
            Ordering::Greater => println!("your guessed number is greater"),
            Ordering::Equal => {
                println!("you are right");
                break;
            }
        }
    }
}