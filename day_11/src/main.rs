use std::{collections::HashMap, fs, iter};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let stones = process_input(&input);
    println!("Part 1: {:}", part1(&stones));
    println!("Part 2: {:}", part2(&stones));
}

fn process_input(input: &str) -> Vec<u64>{
    input.trim().split(" ").map(|num| {num.parse().unwrap()}).collect()
}


fn split_digits(stone: u64) -> Option<(u64, u64)>{
    let s = stone.to_string();
    if s.len() % 2 == 1 {
        return None
    }
    let (half1, half2) = s.split_at(s.len()/2);
    Some((half1.parse().unwrap(), half2.parse().unwrap()))

}

fn blink_rocks( stones: &Vec<u64>, iterations: i32) -> usize{
    let mut stones = stones.clone();
    for _ in 0..iterations {
        let mut new_stones: Vec<u64> = Vec::new();
        for stone in stones {
            if let Some((half1, half2)) = split_digits(stone) {
                new_stones.push(half1);
                new_stones.push(half2);
            } else if stone == 0 {
                new_stones.push(1);
            }else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }
    stones.len()
}

fn part1(stones: &Vec<u64>) -> usize {
    blink_rocks(stones, 25)
}

fn blink_rocks_recur( stone: u64, iterations: i32, memo: &mut HashMap<(u64, i32), u64>) -> u64{
    if iterations == 0 {
        return 1;
    }
    if memo.contains_key(&(stone, iterations)){
        return *memo.get(&(stone, iterations)).unwrap();
    }

    let result;
    if let Some((half1, half2)) = split_digits(stone) {
        result = blink_rocks_recur(half1, iterations-1, memo)
         + blink_rocks_recur(half2, iterations-1, memo);
    } else if stone == 0 {
        result = blink_rocks_recur(1, iterations-1, memo);
    }else {
        result = blink_rocks_recur(stone * 2024, iterations-1, memo);
    }
    memo.insert((stone, iterations), result);
    result
}


fn part2(stones: &Vec<u64>) -> u64 {
    let mut memo = HashMap::new();
    stones.iter().map(|stone| {blink_rocks_recur(*stone, 75, &mut memo)}).sum()
}
