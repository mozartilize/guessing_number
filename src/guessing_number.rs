use rand::Rng;
use std::io::{self, Write};

use tracing::{debug, info, instrument};

#[instrument]
pub fn guessing_number() {
    info!("hello");

    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..10) + 1;
    debug!("{}", number);

    let mut buffer = String::new();
    let mut guessed = false;
    for i in 0..3 {
        if i == 0 {
            print!("Enter your number: ");
        } else {
            print!("Wrong! Try again: ");
        }
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();
        // println!("{}", buffer);
        let input_number: u8 = buffer.trim().parse().unwrap();
        buffer.clear();
        if input_number == number {
            println!("You're correct!");
            guessed = true;
            break;
        }
    }
    if !guessed {
        println!("Aww, you're out of luck! The number is {}", number);
    }
}
