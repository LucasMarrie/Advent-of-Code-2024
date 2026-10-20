use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fs::read_to_string};

fn main() {
    println!("Hello, world!");
    let input = read_to_string("input.txt").unwrap();
    let numbers = input.lines().map(|l| {l.parse().unwrap()}).collect::<Vec<usize>>();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

}

fn mix(num: usize, other: usize) -> usize{
    num ^ other
}

fn prune(num: usize) -> usize {
    num % 16777216
}

fn random_secret(mut num: usize) -> usize{
    num = prune(mix(num, num * 64));
    num = prune(mix(num, num / 32));
    prune(mix(num, num * 2048))
}

fn part1(numbers: &Vec<usize>) -> usize{
    let mut total = 0;
    for num in numbers {
        let mut num = *num;
        for _ in 0..2000 {
            num = random_secret(num);
        }
        total += num;
    }
    total
}

fn part2(numbers: &Vec<usize>) -> usize{
    
    let mut changes_map = HashMap::new();
    for num in numbers {
        let mut changes_detected = HashSet::new();
        let mut q = VecDeque::new();
        let mut num = *num;
        for _ in 0..2000 {
            let prev = num;
            num = random_secret(num);
            let change = (num % 10) as i32 - (prev % 10) as i32;
            q.push_back(change);
            if q.len() > 4 {
                q.pop_front();
                if changes_detected.insert(q.clone()) {
                    *changes_map.entry(q.clone()).or_insert(0) += num % 10;
                }
            }
        }
    }
    let max_entry =  changes_map.iter().max_by(|a, b| {a.1.cmp(b.1)}).unwrap();

    *max_entry.1
}