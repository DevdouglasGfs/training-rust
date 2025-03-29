pub mod validator;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use validator::is_validate_username;

fn main() {
    println!("Welcome to the game Guess the number!");

    let username = String::new();

    println!("Is soo good to play with you!\nTalk me, how can I call you?");

    loop {
        let (name, is_not_valid_username_message) = get_user_name(&username);
        if is_validate_username(name) {
            break;
        };
        println!("{}", &is_not_valid_username_message);
    }

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guest.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win {username}, congratulations!");
                break;
            }
        }
    }
}

fn get_user_name(username: &String) -> (String, String) {
    let mut name = username.clone();
    let is_not_valid_username_message =
        format!("it doesn't look like a name, please told me your name :)");

    io::stdin()
        .read_line(&mut name)
        .expect(&is_not_valid_username_message);
    (name, is_not_valid_username_message)
}
