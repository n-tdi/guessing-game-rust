use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut high_score: u16 = 1000;
    loop {
        println!("Guess MY number");

        let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
        let mut guess_amount: u16 = 0;
        loop {
            println!("Enter Guess:");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Unable to read");

            let guess: u8 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a real number idiot");
                    continue;
                },
            };

            println!("You said: {}", guess);
            guess_amount += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small lmao!"),
                Ordering::Greater => println!("Too big (That's what she said)!"),
                Ordering::Equal => {
                    println!("\nWe have a WINNER!");
                    println!("It took you {} guesses to guess the number.", guess_amount);
                    match guess_amount.cmp(&mut high_score) {
                        Ordering::Less => {
                            println!("NEW HIGH-SCORE {}", guess_amount);
                            high_score = guess_amount;
                        }
                        Ordering::Equal => {
                            println!("You MATCHED your current HIGH-SCORE of {}", high_score)
                        }
                        Ordering::Greater => {
                            println!("You current HIGH-SCORE is {}", high_score)
                        }
                    }
                    break;
                }
            }
        }
        println!("\nWould you like to play again?\n(Y)es (N)o");
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Unable to read");

        if response.trim().to_lowercase() != "y" {
            println!("See you next time");
            break;
        } else {
            println!("Restarting game... \n");
        }
    }
}