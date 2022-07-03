use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess MY number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_amount = 0;

    loop {
        println!("Enter Guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You said: {}", guess);
        guess_amount = guess_amount + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small lmao!"),
            Ordering::Greater => println!("Too big (That's what she said)!"),
            Ordering::Equal => {
                println!("We have a WINNER!");
                println!("It took you {} guesses to guess the number.", guess_amount);
                break;
            }
        }
    }
}