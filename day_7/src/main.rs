use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let args: Vec<Vec<i128>> = process_input(&input);
    println!("part 1: {:}", part1(&args));
    println!("part 2: {:}", part2(&args));
}

fn process_input(input: &str) -> Vec<Vec<i128>>{
    input.lines().map(|line| {
        let mut s = line.to_string();
        s.remove(line.chars().position(|c| {c == ':'}).unwrap());
        s.split(' ').map(|c| {c.parse().unwrap()}).collect()
    }).collect()
}

fn concat_op(mut a: i128, b: i128) -> i128{
    let mut temp_b = b;
    while temp_b > 0 {
        a *= 10;
        temp_b /= 10;
    }
    a + b
}


fn compute_possibility(args: &Vec<i128>, index: usize, total: i128) -> bool {
    if index >= args.len(){
        return args[0] == total
    }
    if total > args[0] {
        return false
    }
    compute_possibility(args, index + 1, total * args[index]) 
    || compute_possibility(args, index + 1, total + args[index])
}

fn part1(args: &Vec<Vec<i128>>) -> i128 {
    args.iter().fold(0,|total, arg| {
        if compute_possibility(arg, 2, arg[1]) {
            total + arg[0]
        }else{
            total
        }
    })
}

fn compute_possibility_concat(args: &Vec<i128>, index: usize, total: i128) -> bool {
    if index >= args.len(){
        return args[0] == total
    }
    if total > args[0] {
        return false
    }
    compute_possibility_concat(args, index + 1, total * args[index]) 
    || compute_possibility_concat(args, index + 1, total + args[index])
    || compute_possibility_concat(args, index + 1, concat_op(total, args[index]))
}

fn part2(args: &Vec<Vec<i128>>) -> i128 {
    args.iter().fold(0,|total, arg| {
        if compute_possibility_concat(arg, 2, arg[1]) {
            total + arg[0]
        }else{
            total
        }
    })
}