use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (patterns, designs) = process_inputs(&input);
    println!("Part 1: {}", part1(&patterns, &designs));
    println!("Part 2: {}", part2(&patterns, &designs));

}

fn process_inputs<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>){
    let mut lines = input.lines();
    let patterns   = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    lines.next();
    let designs = lines.collect::<Vec<_>>();

    (patterns, designs)
}

fn check_possibility(design: &str, patterns: &Vec<&str>, i: usize) -> bool{
    if i == design.len() {
        return true;
    }
    for pattern in patterns {
        if pattern.len() <= design.len() - i && *pattern == &design[i..i+pattern.len()] {
            if check_possibility(design, patterns, i + pattern.len()) {
                return true;
            }
        }
    }

    false
}

fn part1(patterns: &Vec<&str>, designs: &Vec<&str>) -> usize {
    designs.iter().filter(|design| {check_possibility(design, patterns, 0)}).count()
}

fn get_configurations(design: &str, patterns: &Vec<&str>, i: usize, memo: &mut HashMap<usize, usize>) -> usize{
    if i == design.len() {
        return 1;
    }
    if memo.contains_key(&i) {
        return *memo.get(&i).unwrap();
    }
    let mut combinations = 0;
    for pattern in patterns {
        if pattern.len() <= design.len() - i && *pattern == &design[i..i+pattern.len()] {
            combinations += get_configurations(design, patterns, i + pattern.len(), memo);
        }
    }
    memo.insert(i, combinations);
    combinations
}

fn part2(patterns: &Vec<&str>, designs: &Vec<&str>) -> usize {
    designs.iter().map(|design| {let mut memo = HashMap::new(); get_configurations(design, patterns, 0, &mut memo)}).sum()
}
