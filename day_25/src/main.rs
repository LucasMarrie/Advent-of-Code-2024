use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (locks, keys) = process_input(&input);
    println!("Part 1: {}", part1(&locks, &keys));
}

fn process_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut row = 0;
    let mut reading_lock = true;
    let mut elem = vec![-1; 5];
    for line in input.lines()  { 
        if line.len() == 0 {
            if reading_lock {
                locks.push(elem);
            }else{
                keys.push(elem);
            }
            elem = vec![-1; 5];
            row = 0;
            continue;
        }
        if row == 0 {reading_lock = line.chars().nth(0) == Some('.') };
        for (i, c) in line.chars().enumerate() {
            if c == '#' {elem[i] += 1}
        }
        row += 1;
    }

    (locks, keys)


}



fn part1(locks: &Vec<Vec<i32>>,keys: &Vec<Vec<i32>>) -> usize{
    let mut result = 0;
    for lock in locks {
        for key in keys {
            let mut correct = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] > 5 {
                    correct = false;
                    break;
                }
            }
            if correct {
                result += 1;
            }
        }
    }
    result
}