use itertools::Itertools;
use std::cmp::Reverse;

fn main() {
    // Part one
    let lines = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let answer = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max().unwrap();

    println!("Answer: {answer:?}");

    // Part two
    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();

    println!("Answer: {answer:?}");
}
