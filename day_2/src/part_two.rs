use super::*;

#[derive(PartialEq)]
enum Result {
    Lose,
    Draw,
    Win,
}

struct Choice {
    opponent: Option<HandShape>,
    me: Option<Result>,
}

pub fn main() {
    let collection = get_collection("./src/input.txt");

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
                "X" => Some(Result::Lose),
                "Y" => Some(Result::Draw),
                "Z" => Some(Result::Win),
                _ => None,
            };

            Some(Choice { opponent, me })
        })
        .flatten()
        .collect();

    let mut total_score = 0;

    for choice in choices {
        let mut score = 0;

        let opponent_choice = choice.opponent.unwrap();
        let me_choice = choice.me.unwrap();

        let new_choice = if opponent_choice == HandShape::Rock && me_choice == Result::Lose
            || opponent_choice == HandShape::Paper && me_choice == Result::Win
            || opponent_choice == HandShape::Scissors && me_choice == Result::Draw
        {
            Some(HandShape::Scissors)
        } else if opponent_choice == HandShape::Rock && me_choice == Result::Draw
            || opponent_choice == HandShape::Paper && me_choice == Result::Lose
            || opponent_choice == HandShape::Scissors && me_choice == Result::Win
        {
            Some(HandShape::Rock)
        } else if opponent_choice == HandShape::Rock && me_choice == Result::Win
            || opponent_choice == HandShape::Paper && me_choice == Result::Draw
            || opponent_choice == HandShape::Scissors && me_choice == Result::Lose
        {
            Some(HandShape::Paper)
        } else {
            None
        };

        let new_choice = new_choice.unwrap();

        match new_choice {
            HandShape::Rock => score += 1,
            HandShape::Paper => score += 2,
            HandShape::Scissors => score += 3,
        }

        if opponent_choice == new_choice {
            score += 3;
        } else if opponent_choice == HandShape::Rock && new_choice == HandShape::Paper
            || opponent_choice == HandShape::Paper && new_choice == HandShape::Scissors
            || opponent_choice == HandShape::Scissors && new_choice == HandShape::Rock
        {
            score += 6;
        }

        total_score += score;
    }

    println!("Total Score: {}", total_score);
}
