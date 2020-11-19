use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

#[allow(dead_code)]
pub fn run() {
    println!("Welcome to the guessing game!");

    let answer = rand::thread_rng().gen_range(1, 101);

    loop {
        print!("\nEnter a guess: ");
        // Need to flush so it prints immediately
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\nThe number was {}", guess);
                break;
            }
        };
    }
}
