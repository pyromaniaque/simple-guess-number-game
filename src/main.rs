

use rand::Rng;
use std::io::Write;
use std::{thread, time::Duration};
use colored::Colorize;
use crossterm::{
    execute, 
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};

#[derive(Debug)]
enum Comparison {
    Less,
    Greater,
    Equal,
}

enum Turn {
    Player,
    Enemy,
}

fn update_ai_values(ai_min: &mut u32, ai_max: &mut u32, guess: u32, number: u32) {
    if guess < number {
        *ai_min = guess + 1;
    } else if guess > number  {
        *ai_max = guess - 1;
    }
}

fn compare(input_number: u32, number: u32) -> (Comparison, u32) {
    let comparison = if input_number < number {
        Comparison::Less
    } else if input_number > number {
        Comparison::Greater
    } else {
        Comparison::Equal
    };
    (comparison, input_number)
}

fn enemys_turn(ai_min_value: u32, ai_max_value: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let ai_guess: u32 = rng.gen_range(ai_min_value..=ai_max_value);

    ai_guess
}

fn players_turn() -> u32 {
    let mut input = String::new();

    write!(std::io::stdout(), "Enter your guess: ").unwrap();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error in input stream");

    let input: u32 = input.trim().parse().unwrap_or(0);
    // writeln!(std::io::stdout(), "{}", input).unwrap();
    // compare(input, number);

    input
}

fn game(min_value: u32, max_value: u32) {
    let mut ai_min_value: u32 = min_value;
    let mut ai_max_value: u32 = max_value;

    let mut rng = rand::thread_rng();
    let delay = Duration::from_millis(500);

    let number: u32 = rng.gen_range(1..=100);

    let mut turn = Turn::Player;

    let game_start_title = format!(
        "\nGame started, guess the number from {} to {}\n\n",
        min_value, max_value
    ).green();

    write!(std::io::stdout(), "{}", game_start_title).unwrap();
    std::io::stdout().flush().unwrap();

    loop {
        thread::sleep(delay);
        match turn {
            Turn::Player => {
                let players_turn_title = format!(
                    "\nPlayer's turn!\n"
                ).blue().bold();

                write!(std::io::stdout(), "{}", players_turn_title).unwrap();
                std::io::stdout().flush().unwrap();

                let player_guess = players_turn();

                if player_guess >= min_value && player_guess <= max_value {
                    let (result, _) = compare(player_guess, number);
                    thread::sleep(delay);
                    match result {
                        Comparison::Less => {
                            write!(std::io::stdout(), "guessed is less\n").unwrap()
                        },
                        Comparison::Greater => {
                            write!(std::io::stdout(), "guessed number is greater\n").unwrap()
                        },

                        Comparison::Equal => {
                            let guessed_title = format!(
                                "You guessed the random number\n Player Won!!!\n"
                            ).green();

                            write!(std::io::stdout(), "{}\n", guessed_title).unwrap();
                            std::io::stdout().flush().unwrap();
                            break;
                        },
                    }
                    turn = Turn::Enemy;
                } else {
                    let wrong_number_title = format!(
                        "You typed the wrong number, guess must be between {} and {}\n",
                        min_value, max_value
                    ).red();

                    write!(std::io::stdout(), "{}", wrong_number_title).unwrap();
                }
            }

            Turn::Enemy => {
                let enemys_turn_title = format!(
                    "\nEnemy's turn!\n"
                ).red().bold();

                write!(std::io::stdout(), "{}", enemys_turn_title).unwrap();
                std::io::stdout().flush().unwrap();

                let enemys_guess = enemys_turn(ai_min_value, ai_max_value);
                let (result, _) = compare(enemys_guess, number);
                thread::sleep(delay);
                match result {
                    Comparison::Less => {
                        write!(std::io::stdout(), "enemys guessed is less\n").unwrap();
                        update_ai_values(&mut ai_min_value, &mut ai_max_value, enemys_guess, number);
                    },
                    Comparison::Greater => {
                        write!(std::io::stdout(), "enemys guessed number is greater\n").unwrap();
                        update_ai_values(&mut ai_min_value, &mut ai_max_value, enemys_guess, number);
                    },

                    Comparison::Equal => {
                        let guessed_title = format!(
                            "Enemy guessed the random number\n Enemy Won!!!\n"
                        ).red();

                        write!(std::io::stdout(), "{}\n", guessed_title).unwrap();
                        std::io::stdout().flush().unwrap();
                        break;
                    },
                }
                turn = Turn::Player;
            }
        }

        std::io::stdout().flush().unwrap();
    }
    let final_title = format!("\n The number was: {}\n\n", number).blue().bold();
    write!(std::io::stdout(), "{}", final_title).unwrap();
    std::io::stdout().flush().unwrap();
}

fn main() {

    execute!(
        std::io::stdout(), 
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();

    let min_value = 1u32;
    let max_value = 100u32;

    game(min_value, max_value);
    std::io::stdout().flush().unwrap();
}
