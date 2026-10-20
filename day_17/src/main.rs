use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input).join(","));
    println!("Part 2: {}", part2(&input));
}

fn execute(instructions: &Vec<usize>, mut registers: [usize; 3]) -> Vec<usize> {
    let mut outs = Vec::new();
    let mut pc = 0;
    while (pc as usize) < instructions.len() {
        let ins = instructions[pc as usize];
        let operand = instructions[pc as usize + 1];
        let combo = if operand < 4 {operand} else {registers[operand-4]};
        match ins {
            0 => registers[0] = registers[0] / (2 as usize).pow(combo as u32),
            1 => registers[1] = registers[1] ^ operand,
            2 => registers[1] = combo % 8,
            3 => if registers[0] != 0 {pc = operand as i32 - 2},
            4 => registers[1] = registers[1] ^ registers[2],
            5 => outs.push(combo % 8),
            6 => registers[1] = registers[0] / (2 as usize).pow(combo as u32),
            7 => registers[2] = registers[0] / (2 as usize).pow(combo as u32),
            _ => ()
        };
        pc += 2;
    }
    outs
}

fn part1(input: &str) -> Vec<String>{
    let mut lines = input.lines();
    let a: usize = lines.next().unwrap().split(":").skip(1).next().unwrap().trim().parse().unwrap();
    let b: usize = lines.next().unwrap().split(":").skip(1).next().unwrap().trim().parse().unwrap();
    let c: usize = lines.next().unwrap().split(":").skip(1).next().unwrap().trim().parse().unwrap();

    let instructions = lines.skip(1).next().unwrap().split(":").skip(1).next().unwrap()
        .split(",").map(|c| {c.trim().parse::<usize>().unwrap()}).collect::<Vec<_>>();
    let registers = [a, b, c];
    let res = execute(&instructions, registers);
    res.iter().map(|val| {val.to_string()}).collect()
}


fn solve(instructions: &Vec<usize>, prefix: usize, i: usize) -> Option<usize> {
    for digit in 0..8 {
        let a =  prefix * 8 + digit;
        let res = execute(instructions, [a, 0, 0]);
        if res[0] == instructions[i]{
            if i == 0 {
                return Some(a);
            }
            if let Some(res) = solve(instructions, a, i-1){
                return Some(res);
            }
        }
    }
    None
}

fn part2(input: &str) -> usize{
    let instructions = input.lines().skip(4).next().unwrap().split(":").skip(1).next().unwrap()
        .split(",").map(|c| {c.trim().parse::<usize>().unwrap()}).collect::<Vec<_>>();
    
    solve(&instructions, 0, instructions.len()-1).unwrap()
}
