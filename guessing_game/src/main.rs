use std::io;

fn main() {
    println!("### Guess the Number! ###");
    println!("### Enter your number guess below: ###");

    // mut means mutable
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Unable to read your entry...");

    println!("### You guessed: {}", guess);
}
