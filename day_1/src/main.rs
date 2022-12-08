use std::fs::*;
use std::io::*;

#[derive(Debug)]
struct Elf {
    calories: i32,
}

fn main() {
    let file = File::open("./src/input.txt").expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut elfs: Vec<Elf> = Vec::new();
    let mut sum: i32 = 0;

    for line in lines {
        if let Ok(x) = line {
            match x.parse::<i32>() {
                Ok(n) => sum += n,
                Err(_) => {
                    elfs.push(Elf { calories: sum });
                    sum = 0;
                }
            };
        }
    }

    let mut fattest_elf = &Elf { calories: 0 };

    for elf in &elfs {
        if elf.calories > fattest_elf.calories {
            fattest_elf = elf;
        }
    }
    println!("Fattest elf: {:?}", fattest_elf);

    let second_fattest_elf = next_fattest_elf(&elfs, &fattest_elf);
    println!("Second fattest elf: {:?}", second_fattest_elf);

    let third_fattest_elf = next_fattest_elf(&elfs, &second_fattest_elf);
    println!("Third fattest elf: {:?}", third_fattest_elf);

    let sum = fattest_elf.calories + second_fattest_elf.calories + third_fattest_elf.calories;
    println!("Sum: {}", sum);
}

fn next_fattest_elf<'a>(elfs: &'a Vec<Elf>, fatter_elf: &'a Elf) -> &'a Elf {
    let mut next_fattest_elf = &Elf { calories: 0 };

    for elf in elfs {
        if elf.calories > next_fattest_elf.calories && elf.calories < fatter_elf.calories {
            next_fattest_elf = elf;
        }
    }
    next_fattest_elf
}
