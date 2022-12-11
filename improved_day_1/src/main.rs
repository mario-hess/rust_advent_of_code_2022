fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let max = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max();

    println!("Max = {max:?}");
}
