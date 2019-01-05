use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("### Guess the Number! ###");
    let secret_num = rand::thread_rng().gen_range(1, 10);

    loop {
        // mut means mutable
        let mut guess = String::new();
        println!("### Enter your number guess between 1 and 10 below: ###");
        io::stdin().read_line(&mut guess) // & means reference here, like a pointer in C
            .expect("Unable to read your entry...");
        println!("### You guessed: {}", guess);

        // This is shadowing- multiple representations of the same value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a number, try again");
                continue
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You got it!!");
                break;
            }
        }
    }
}
