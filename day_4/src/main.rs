use std::fs::*;
use std::io::*;

fn main() {
    let file = File::open("./src/input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut data_list = get_data_list(&mut lines);

    let sum = calc(&mut data_list, &contains);
    println!("Contains: {}", sum);

    let sum = calc(&mut data_list, &overlaps);
    println!("Overlaps: {}", sum);
}

fn get_data_list(lines: &mut Lines<BufReader<File>>) -> Vec<Vec<Vec<u32>>> {
    lines
        .into_iter()
        .map(|l| {
            l.unwrap()
                .split(",")
                .map(str::to_owned)
                .map(|j| {
                    j.split("-")
                        .map(|k| k.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn calc(data_list: &mut Vec<Vec<Vec<u32>>>, f: &dyn Fn(u32, u32, u32, u32) -> bool) -> u32 {
    let mut counter = 0;

    for data_set in data_list {
        if f(
            data_set[0][0],
            data_set[0][1],
            data_set[1][0],
            data_set[1][1],
        ) {
            counter += 1;
        }
    }

    counter
}

fn contains(a: u32, b: u32, x: u32, y: u32) -> bool {
    a <= x && b >= y || a >= x && b <= y
}

fn overlaps(a: u32, b: u32, x: u32, y: u32) -> bool {
    (a <= x && b >= y || a >= x && b <= y)
        || (a <= x && b >= x)
        || (a <= y && b >= y)
        || (a >= x && b <= x)
        || (a >= y && b <= y)
}
