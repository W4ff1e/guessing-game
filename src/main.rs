use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    // Test Code: (aaron-bond.better-comments)
    //// println!("The secret number is: {secret_number}");

    loop {
        println!("Please enter your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        let mut exit: String = String::new();
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Press \"Enter\" to quit!");
                io::stdin()
                    .read_line(&mut exit)
                    .expect("Failed to read input!");
                break;
            }
        };
    }
}
