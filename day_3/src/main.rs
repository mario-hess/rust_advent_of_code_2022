use std::fs::*;
use std::io::*;

#[derive(PartialEq)]
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

#[derive(Clone)]
struct Group {
    items: Vec<Vec<char>>,
}

impl Group {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

fn main() {
    let path = "./src/input.txt";

    // Part one
    let lines = get_lines(path);
    let collection = get_matching_items(lines);
    let sum = calculate_priorities(collection);

    println!("Sum: {}", sum);

    // Part two
    let lines = get_lines(path);
    let groups = get_groups(lines);
    let chars = get_group_match(groups.clone());
    let sum = calculate_priorities(chars);

    println!("Sum: {}", sum);
}

fn get_group_match(groups: Vec<Group>) -> Vec<char> {
    let mut matched_list: Vec<char> = Vec::new();

    for group in groups {
        let first: Vec<char> = group.items[0].clone();
        let second: Vec<char> = group.items[1].clone();
        let third: Vec<char> = group.items[2].clone();

        let mut possible_match: Vec<char> = Vec::new();
        let mut matched_char: char = ' ';

        for items in first.clone() {
            for item in second.clone() {
                if item == items {
                    possible_match.push(item);
                }
            }
        }

        for items in possible_match {
            for item in third.clone() {
                if item == items {
                    matched_char = item;
                    break;
                }
            }
        }

        matched_list.push(matched_char);
    }

    matched_list
}

fn get_groups(lines: Lines<BufReader<File>>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let mut group: Group = Group::new();

    let mut counter = 0;
    for line in lines {
        let mut l: Vec<char> = Vec::new();

        for c in line.unwrap().chars() {
            l.push(c);
        }

        counter += 1;
        group.items.push(l);

        if counter == 3 {
            groups.push(group.clone());
            group.items.clear();
            counter = 0;
        }
    }

    groups
}

fn calculate_priorities(collection: Vec<char>) -> u32 {
    let mut sum = 0;

    for item in collection {
        let i = item as u32;

        if i < 97 {
            sum += i - 38;
        } else {
            sum += i - 96;
        }
    }

    sum
}

fn get_matching_items(lines: Lines<BufReader<File>>) -> Vec<char> {
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

    collection
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

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("Unable to open file.");
    let lines = BufReader::new(file).lines();

    lines
}
