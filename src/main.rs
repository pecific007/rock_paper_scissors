#[warn(clippy::pedantic)]
use rand::RngExt;
use std::io;
use std::io::Write;
use std::matches;

enum Choices {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

impl Choices {
    fn winner(&self, option: Choices) -> bool {
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

    let winner = user.winner(comp);
    if winner {
        println!("You won!");
        return;
    } else {
        println!("Computer won!");
        return;
    }
}

fn get_user_ip() -> Choices {
    let mut input = String::new();
    print!("Choose: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't process input.");

    match input.trim().to_lowercase().as_str() {
        "rock" => Choices::Rock,
        "paper" => Choices::Paper,
        "scissors" => Choices::Scissors,
        _ => Choices::Invalid,
    }
}

fn get_comp_ip() -> Choices {
    let index = rand::rng().random_range(0..3);
    match index {
        0 => Choices::Rock,
        1 => Choices::Paper,
        2 => Choices::Scissors,
        _ => Choices::Invalid,
    }
}

fn get_string_from_choice(c: &Choices) -> &str {
    match c {
        Choices::Rock => "Rock",
        Choices::Paper => "Paper",
        Choices::Scissors => "Scissors",
        Choices::Invalid => "Invalid",
    }
}
