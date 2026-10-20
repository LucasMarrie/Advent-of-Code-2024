use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {:}", part1(&input));
    println!("Part 2: {:}", part2(&input));
}

fn part1(input: &str) -> i32{
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for caps in re.captures_iter(input){
        let x: i32 = caps[1].parse().unwrap();
        let y: i32 = caps[2].parse().unwrap();
        sum += x * y;
    };
    sum
}

fn part2(input: &str) -> i32{
    let mut sum = 0;
    let mut do_mul = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    for caps in re.captures_iter(input){
        let cap_str = caps.get(0).unwrap().as_str();
        if cap_str == "do()"{
            do_mul = true;
        } else if cap_str == "don't()"{
            do_mul = false;
        } else if do_mul{
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            sum += x * y;
        }
    };
    sum
}
