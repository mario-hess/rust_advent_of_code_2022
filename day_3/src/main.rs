use std::fs::*;
use std::io::*;

#[derive(Debug)]
struct Rucksack {
    items: Vec<char>,
    compartment_one: Vec<char>,
    compartment_two: Vec<char>,
}

impl Rucksack {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            compartment_one: Vec::new(),
            compartment_two: Vec::new(),
        }
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Unable to open file.");
    let lines = BufReader::new(file).lines();

    let rucksacks = sort_compartments(get_rucksacks(lines));

    let mut collection: Vec<char> = Vec::new();

    for rucksack in &rucksacks {
        let mut result: char = ' ';
        for i in &rucksack.compartment_one {
            for j in &rucksack.compartment_two {
                if i == j {
                    result = *j;
                    break;
                }
            }
        }
        collection.push(result);
    }
    
    collection.sort();

    let mut sum = 0;

    for item in collection {
        let i = item as u32;
        
        if i < 97 {
            sum += i - 38;
        }
        else {
            sum += i - 96; 
        }    
    }

    println!("Sum: {}", sum);
}

fn sort_compartments(rucksacks: Vec<Rucksack>) -> Vec<Rucksack> {
    let mut rucksacks = rucksacks;

    for rucksack in &mut rucksacks {
        let split_at = rucksack.items.len() / 2;

        let mut counter = 0;

        for item in &rucksack.items {
            if counter < split_at {
                rucksack.compartment_one.push(*item);
            } else {
                rucksack.compartment_two.push(*item);
            }

            counter += 1;
        }
    }

    rucksacks
}

fn get_rucksacks(lines: Lines<BufReader<File>>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in lines {
        let mut rucksack = Rucksack::new();

        for c in line.unwrap().chars() {
            rucksack.items.push(c);
        }

        rucksacks.push(rucksack);
    }

    rucksacks
}
