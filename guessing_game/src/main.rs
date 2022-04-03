use std::io;

fn main() {
    // generate a random number
    let random_number = 50;
    // ask the player to type in a guess
    println!("Guess which number I'm thinking!");
    // get user input
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // compare the guess to the random number
    // Loop
    loop {
        println!("Input your guess!");
        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => println!("You win!"),
        }
    }
    
    // three cases:
    // 1. match
    //     - print success and end game
    // 2. too high, then tell user to guess again
    // 3. too low, then tell user to guess again
}
