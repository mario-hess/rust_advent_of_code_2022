use std::collections::HashSet;
use std::fs;

fn is_unique(chars: &[char]) -> bool {
    let set = chars.iter().collect::<HashSet<_>>();
    set.len() == chars.len()
}

fn find_unique_window(data: Vec<char>) -> Option<usize> {
    for (i, window) in data.windows(14).enumerate() {
        if is_unique(window) {
            return Some(i + window.len())
        }
    }

    None
}

fn main() {
    let content = fs::read_to_string("./src/input.txt").unwrap();
    let data: Vec<char> = content.chars().collect();

    let index = find_unique_window(data).expect("Not found.");
    println!("{:?}", index);
}
