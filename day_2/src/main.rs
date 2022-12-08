use std::fs::*;
use std::io::*;

mod part_one;
mod part_two;

#[derive(PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    part_one::main();
    part_two::main();
}

pub fn get_collection(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file.");
    let lines = BufReader::new(file).lines();

    let mut collection: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(x) = line {
            collection.push(x);
        }
    }
    collection
}
