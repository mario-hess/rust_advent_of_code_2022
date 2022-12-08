use std::fs::*;
use std::io::*;

#[derive(Debug, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Choice {
    opponent: Option<HandShape>,
    me: Option<HandShape>,
}

fn main() {
    let choices = get_choices("./src/input.txt");

    let mut total_score = 0;

    for choice in choices {
        let mut score = 0;

        let opponent_choice = choice.opponent.unwrap();
        let me_choice = choice.me.unwrap();

        match me_choice {
            HandShape::Rock => score += 1,
            HandShape::Paper => score += 2,
            HandShape::Scissors => score += 3,
        }

        if opponent_choice == me_choice {
            score += 3;
        } else if opponent_choice == HandShape::Rock && me_choice == HandShape::Paper
            || opponent_choice == HandShape::Paper && me_choice == HandShape::Scissors
            || opponent_choice == HandShape::Scissors && me_choice == HandShape::Rock
        {
            score += 6;
        }

        total_score += score;
    }

    println!("Total Score: {}", total_score);
}

fn get_choices(path: &str) -> Vec<Choice> {
    let file = File::open(path).expect("Unable to open file.");
    let lines = BufReader::new(file).lines();

    let mut collection: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(x) = line {
            collection.push(x);
        }
    }

    let choices: Vec<Choice> = collection
        .iter()
        .map(|s| {
            let mut s = s.split(' ');

            let opponent = s.next()?;
            let opponent = match opponent {
                "A" => Some(HandShape::Rock),
                "B" => Some(HandShape::Paper),
                "C" => Some(HandShape::Scissors),
                _ => None,
            };

            let me = s.next()?;
            let me = match me {
                "X" => Some(HandShape::Rock),
                "Y" => Some(HandShape::Paper),
                "Z" => Some(HandShape::Scissors),
                _ => None,
            };

            Some(Choice { opponent, me })
        })
        .flatten()
        .collect();

    choices
}
