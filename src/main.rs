mod error_trackers_model;
pub mod validator;
use error_trackers_model::unable_to_read_read_line_message;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::Rng;
use std::cmp::Ordering;
use std::ops::Sub;
use std::{io, usize};
use validator::is_validate_username;

use std::str::FromStr;

#[derive(PartialEq, Debug, EnumIter)]
enum GameLevels {
    EASY,
    MEDIUM,
    HARD,
}

impl FromStr for GameLevels {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "easy" => Ok(GameLevels::EASY),
            "medium" => Ok(GameLevels::MEDIUM),
            "hard" => Ok(GameLevels::HARD),
            _ => Err(format!("'{}' is not a valid game level", s)),
        }
    }
}

use std::fmt;

impl fmt::Display for GameLevels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameLevels::EASY => write!(f, "Easy Mode"),
            GameLevels::MEDIUM => write!(f, "Medium Mode"),
            GameLevels::HARD => write!(f, "Hard Mode"),
        }
    }
}

fn main() {
    println!("Hi! I'm Angela and I love to play!\n");
    println!("Let's see how're your lucky?! 🪄");

    let username = String::new();
    println!("I want too much play... Let's you want to play with me? but before");
    println!("Tell me, how can I call you? ✨");

    let data = get_username(&username);

    println!("\nLet's go to play {} 🚀?\n", data.name.trim());
    let prompt = prompt_the_game_mode();

    let game_definitions = define_game_mode(prompt.game_mode);

    println!("\n============================");
    println!("\nRight! Let's start the game!");
    println!("\n============================");

    println!(
        "\nYou have selected the {:?} mode!\n",
        game_definitions.level
    );

    start_game(
        game_definitions.view_space,
        &data.name,
        game_definitions.level,
    );
}

struct GetUserNameResult {
    name: String,
}

fn get_username(username: &String) -> GetUserNameResult {
    let mut name = username.clone();

    let is_not_valid_username_message =
        format!("it doesn't look like a name, please told me your name ❤️✨");

    'take_a_valid_username: loop {
        let mut scoped_name = String::from(&name);

        io::stdin()
            .read_line(&mut scoped_name)
            .expect(&unable_to_read_read_line_message())
            .to_string();

        if is_validate_username(scoped_name.clone()) {
            let first_letter = &scoped_name.split_at(1).0.to_ascii_uppercase();
            name = format!("{}{}", first_letter, scoped_name.split_at(1).1);
            break 'take_a_valid_username;
        };
        println!("{}", &is_not_valid_username_message);
    }

    GetUserNameResult { name }
}

fn start_game(max_view_space: u8, username: &String, level: GameLevels) {
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=max_view_space as u32);

        if level != GameLevels::HARD {
            get_secret_number_tip(max_view_space);
        }

        println!("Please guess a number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect(&unable_to_read_read_line_message());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{} also don't is part of the game... but justtry again!\n",
                    { guess.replace("\n", "") }
                );
                continue;
            }
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! The correct answer is {}", &secret_number),
            Ordering::Greater => println!("Too big! The correct answer is {}", &secret_number),
            Ordering::Equal => {
                let username_formatted = username.replace('\n', "");
                println!("🎊 Wow You win {username_formatted}, congratulations!");
                break;
            }
        }
    }
}

struct PromptTheGameModeResult {
    game_mode: GameLevels,
}
fn prompt_the_game_mode() -> PromptTheGameModeResult {
    println!("Select an game mode:");
    let mut modes = Vec::new();
    GameLevels::iter().for_each(|mode| modes.push(mode));
    modes.iter().for_each(|mode| {
        if *mode == GameLevels::EASY {
            println!(
                "{}",
                format!(
                    "😊 {}: it's the better mode to try your lucky. (Default mode)",
                    mode
                )
            );
            println!("\t💬 You'll receive a tip of the response\n");
        } else if *mode == GameLevels::MEDIUM {
            println!(
                "{}",
                format!(
                    "🫣  {}: Woo! You have very much lucky or you are trying your lucky with me?!",
                    mode
                )
            );
            println!("\t💬 You'll receive a tip of the response\n");
        } else {
            println!("{}", format!("💀 {}: Oh my god! What's you?!", mode));
            println!("\t❌ You don't will receive a tip of the response.\n");
        }
    });

    let mut user_input = String::new();

    'define_the_game_level: loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect(&unable_to_read_read_line_message());
        let local_choice = user_input.replace('\n', "");

        let was_provided_just_numbers = local_choice.trim().chars().all(|char| !!char.is_numeric());
        let is_empty_input = local_choice
            .trim()
            .chars()
            .all(|char| !!char.is_whitespace());

        if was_provided_just_numbers {
            let choice = local_choice.parse::<usize>();

            if choice.is_ok() {
                let choice = choice.unwrap_or_else(|_| 1).sub(1);

                let is_valid_mode = !modes.get(choice).is_none();
                if is_valid_mode {
                    let is_easy_mode = choice == 0;
                    let is_medium_mode = choice == 1;

                    if is_easy_mode {
                        break 'define_the_game_level PromptTheGameModeResult {
                            game_mode: GameLevels::EASY,
                        };
                    } else if is_medium_mode {
                        break 'define_the_game_level PromptTheGameModeResult {
                            game_mode: GameLevels::MEDIUM,
                        };
                    } else {
                        break 'define_the_game_level PromptTheGameModeResult {
                            game_mode: GameLevels::HARD,
                        };
                    }
                };
            }
        }

        if is_empty_input {
            break 'define_the_game_level PromptTheGameModeResult {
                game_mode: GameLevels::EASY,
            };
        }

        println!(
            "Please choose only some of the proposed answers. '{}' isn't something that this game has 😢.",
            local_choice.replace("\n", "")
        );
        user_input.clear();
    }
}

struct DefineGameModeResult {
    view_space: u8,
    level: GameLevels,
}
fn define_game_mode(level: GameLevels) -> DefineGameModeResult {
    let is_easy_mode = level == GameLevels::EASY;
    let is_medium_mode = level == GameLevels::MEDIUM;
    let is_hard_mode = level == GameLevels::HARD;

    let mut max_view_space: u8 = 0;

    if is_easy_mode {
        max_view_space = 3
    }
    if is_medium_mode {
        max_view_space = 5
    }
    if is_hard_mode {
        max_view_space = 10
    }
    return DefineGameModeResult {
        view_space: max_view_space,
        level,
    };
}

fn get_secret_number_tip(max_view_space: u8) {
    println!("=====================================================");
    println!(
        ">> Tip 🫣: the answer maybe an number between 1 and {}",
        &max_view_space
    );
    println!("=====================================================");
    println!("\n");
}
