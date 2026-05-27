use rand::RngExt;
use std::io;
use std::matches;

#[allow(nonstandard_style)]
enum Choices {
    rock,
    paper,
    scissors,
    invalid,
}

impl Choices {
    fn winner(&self, option: Choices) -> bool {
        matches!(
            (self, option),
            (Choices::rock, Choices::scissors)
                | (Choices::paper, Choices::rock)
                | (Choices::scissors, Choices::paper)
        )
    }
}

fn main() {
    println!("---------/ ROCK, PAPER, scissors /---------");
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
    println!("Choose: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't process input.");
    input = input.trim().to_lowercase();

    if input == String::from("rock") {
        return Choices::rock;
    } else if input == String::from("paper") {
        return Choices::paper;
    } else if input == String::from("scissors") {
        return Choices::scissors;
    } else {
        return Choices::invalid;
    }
}

fn get_comp_ip() -> Choices {
    let index = rand::rng().random_range(0..3);
    match index {
        0 => Choices::rock,
        1 => Choices::paper,
        2 => Choices::scissors,
        _ => Choices::invalid,
    }
}

fn get_string_from_choice(c: &Choices) -> &str {
    match c {
        Choices::rock => "Rock",
        Choices::paper => "Paper",
        Choices::scissors => "Scissors",
        Choices::invalid => "Invalid",
    }
}
