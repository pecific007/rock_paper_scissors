#[warn(clippy::pedantic)]
use rand::RngExt;
use std::io;
use std::io::Write;
use std::matches;

enum Choices {
    Rock,
    Paper,
    Scissors,
}

impl Choices {
    fn winner(&self, option: &Choices) -> bool {
        matches!(
            (self, option),
            (Choices::Rock, Choices::Scissors)
                | (Choices::Paper, Choices::Rock)
                | (Choices::Scissors, Choices::Paper)
        )
    }
}

fn main() {
    println!("---------/ ROCK, PAPER, SCISSORS /---------");
    let user = get_user_ip();
    let comp = get_comp_ip();

    let user_choice = get_string_from_choice(&user);
    let comp_choice = get_string_from_choice(&comp);

    if user_choice == "Invalid" {
        println!("Invalid Choice!");
        return;
    }
    if comp_choice == "Invalid" {
        println!("Something went wrong!");
        return;
    }

    println!("You chose: {user_choice}");
    println!("Computer chose: {comp_choice}");

    if comp_choice == user_choice {
        println!("It's a tie!");
        return;
    }

    match (user, comp) {
        (Some(user), Some(comp)) => {
            let winner = user.winner(&comp);
            if winner {
                println!("You won!");
            } else {
                println!("Computer won!");
            }
        }
        _ => {
            println!("Something went wrong!")
        }
    }
}

fn get_user_ip() -> Option<Choices> {
    let mut input = String::new();
    print!("Choose: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't process input.");

    match input.trim().to_lowercase().as_str() {
        "rock" => Some(Choices::Rock),
        "paper" => Some(Choices::Paper),
        "scissors" => Some(Choices::Scissors),
        _ => None,
    }
}

fn get_comp_ip() -> Option<Choices> {
    let index = rand::rng().random_range(0..3);
    match index {
        0 => Some(Choices::Rock),
        1 => Some(Choices::Paper),
        2 => Some(Choices::Scissors),
        _ => None,
    }
}

fn get_string_from_choice(c: &Option<Choices>) -> String {
    match c {
        Some(Choices::Rock) => String::from("Rock"),
        Some(Choices::Paper) => String::from("Paper"),
        Some(Choices::Scissors) => String::from("Scissors"),
        None => String::from("Invalid"),
    }
}
