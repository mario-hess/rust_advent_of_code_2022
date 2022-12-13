use super::*;

struct Choice {
    opponent: Option<HandShape>,
    me: Option<HandShape>,
}

pub fn main() {
    let collection = get_collection("./src/input.txt");

    let choices: Vec<Choice> = collection
        .iter()
        .filter_map(|s| {
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
        .collect();

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
