use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    print!("secret number: {}", secret_number);

    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
