use std::fs::*;
use std::io::*;

fn main() {
    let file = File::open("./src/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut extracted_containership = extract_containership(&mut reader);
    extracted_containership.reverse();

    let indexes = get_indexes(&extracted_containership);
    extracted_containership.remove(0);

    let parsed_containership = parse_containership(indexes, extracted_containership);
    let parsed_digits = parse_digits(reader);

    // Part one
    let result = cratemover_9000(&parsed_containership, &parsed_digits);
    println!("Result from CrateMover 9000: {:?}", result);

    // Part two
    let result = cratemover_9001(&parsed_containership, &parsed_digits);
    println!("Result from CrateMover 9001: {:?}", result);
}

fn extract_containership(reader: &mut BufReader<File>) -> Vec<String> {
    let mut containership: Vec<String> = Vec::new();

    for _ in 0..10 {
        if let Some(line) = reader
            .lines()
            .take_while(|line| {
                let line: String = line.as_ref().unwrap().to_string();
                !line.is_empty()
            })
            .next()
        {
            containership.push(line.unwrap());
        } else {
            break;
        }
    }

    containership
}

fn get_indexes(extracted_containership: &[String]) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for (index, char) in extracted_containership[0].chars().enumerate() {
        if !char.is_whitespace() {
            indexes.push(index);
        }
    }

    indexes
}

fn parse_containership(
    indexes: Vec<usize>,
    extracted_containership: Vec<String>,
) -> Vec<Vec<char>> {
    let mut parsed_containership: Vec<Vec<char>> = Vec::new();

    for index in indexes {
        let mut container_row: Vec<char> = Vec::new();

        for i in extracted_containership.iter() {
            let mut line = i.chars().clone();
            if let Some(x) = line.nth(index) {
                if !x.is_whitespace() {
                    container_row.push(x);
                }
            }
        }

        parsed_containership.push(container_row);
    }

    parsed_containership
}

fn parse_digits(reader: BufReader<File>) -> Vec<Vec<usize>> {
    let mut parsed_digits: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines() {
        let mut digits = Vec::new();
        let mut group = Vec::new();
        let line = line.unwrap();

        for c in line.chars() {
            if c.is_numeric() {
                group.push(c);
            } else if !group.is_empty() {
                let n: usize = group
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                digits.push(n);
                group.clear();
            } else {
                continue;
            }
        }

        if !group.is_empty() {
            let n: usize = group
                .iter()
                .map(|c| c.to_string())
                .collect::<String>()
                .parse()
                .unwrap();
            digits.push(n);
        }

        parsed_digits.push(digits);
    }

    parsed_digits
}

fn cratemover_9000(
    parsed_containership: &[Vec<char>],
    parsed_digits: &Vec<Vec<usize>>,
) -> Vec<char> {
    let mut parsed_containership = parsed_containership.to_owned();

    for digits in parsed_digits {
        let move_count = digits[0];
        let from = digits[1] - 1;
        let to = digits[2] - 1;

        for _ in 0..move_count {
            let mut from_clone = parsed_containership[from].clone();
            from_clone.reverse();
            parsed_containership[to].push(from_clone[0]);
            parsed_containership[from].pop();
        }
    }

    let mut containers_on_top: Vec<char> = Vec::new();

    for container_row in parsed_containership {
        let mut row = container_row.clone();
        row.reverse();
        containers_on_top.push(row[0]);
    }

    containers_on_top
}

fn cratemover_9001(
    parsed_containership: &[Vec<char>],
    parsed_digits: &Vec<Vec<usize>>,
) -> Vec<char> {
    let mut parsed_containership = parsed_containership.to_owned();

    for digits in parsed_digits {
        let move_count = digits[0];
        let from = digits[1] - 1;
        let to = digits[2] - 1;

        let mut current_pick: Vec<char> = Vec::new();

        for _ in 0..move_count {
            let mut from_clone = parsed_containership[from].clone();
            from_clone.reverse();
            current_pick.push(from_clone[0]);
            parsed_containership[from].pop();
        }

        current_pick.reverse();

        for i in current_pick.iter().take(move_count) {
            parsed_containership[to].push(*i);
        }
    }

    let mut containers_on_top: Vec<char> = Vec::new();

    for container_row in parsed_containership {
        let mut row = container_row.clone();
        row.reverse();
        containers_on_top.push(row[0]);
    }

    containers_on_top
}
