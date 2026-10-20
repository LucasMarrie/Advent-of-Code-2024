use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let machines = process_input(&input);
    println!("Part 1: {:}", part1(&machines));
    println!("Part 2: {:}", part2(&machines));
}

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64)
}

fn process_input(input: &str) -> Vec<Machine>{
    let mut machines = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for chunk in lines.chunks(4) {
        let a  = chunk[0].split(':').skip(1).next().unwrap().split(",").map(|part| {
            part.split('+').skip(1).next().unwrap().parse().unwrap()
        }).collect::<Vec<_>>();
        let b = chunk[1].split(':').skip(1).next().unwrap().split(",").map(|part| {
            part.split('+').skip(1).next().unwrap().parse().unwrap()
        }).collect::<Vec<_>>();
        let p  = chunk[2].split(':').skip(1).next().unwrap().split(",").map(|part| {
            part.split('=').skip(1).next().unwrap().parse().unwrap()
        }).collect::<Vec<_>>();
        machines.push(Machine {a: (a[0], a[1]), b: (b[0], b[1]), prize: (p[0], p[1])});
    };
    machines
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_reachable(a: (u64, u64), b: (u64, u64), p: (u64, u64)) -> Option<u64> {
    let (dx_a, dy_a) = a;
    let (dx_b, dy_b) = b;
    let (px, py) = p;

    let mut min_cost = None;

    for a in 0..=100 {
        for b in 0..=100 {
            let x = a * dx_a + b * dx_b;
            let y = a * dy_a + b * dy_b;

            if x == px && y == py {
                let cost = 3 * a + 1 * b;
                min_cost = Some(match min_cost {
                    Some(current) => cmp::min(current, cost),
                    None => cost,
                });
            }
        }
    }

    min_cost
}


fn part1(machines: &Vec<Machine>) -> i64{
    machines.iter().map(|machine| {min_tokens_to_reach(machine.a.0 as i64, machine.a.1 as i64,
        machine.b.0 as i64, machine.b.1 as i64, machine.prize.0 as i64,  machine.prize.1 as i64)}).sum()
}

// Helper function: Extended Euclidean Algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extended_gcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

// Solve the Diophantine equation for both axes and minimize cost
fn min_tokens_to_reach(
    dx_a: i64,
    dy_a: i64,
    dx_b: i64,
    dy_b: i64,
    px: i64,
    py: i64,
) -> i64 {
    let a = (dy_b *px
        - dx_b * py)
        / (dx_a * dy_b
            - dy_a * dx_b);
    let b = (px - dx_a * a) / dx_b;

    if a <= 0
        || b <= 0
        || !(px == dx_a * a + dx_b * b && py == dy_a * a + dy_b * b)
    {
        0
    } else {
        3 * a + b
    }
}


fn part2(machines: &Vec<Machine>) -> i64 {
    let machines: Vec<Machine> = machines.iter().map( |machine| {
        Machine {a: machine.a, b: machine.b, prize: (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000)}
    }).collect();
    machines.iter().map(|machine| {min_tokens_to_reach(machine.a.0 as i64, machine.a.1 as i64,
        machine.b.0 as i64, machine.b.1 as i64, machine.prize.0 as i64,  machine.prize.1 as i64)}).sum::<i64>()
}