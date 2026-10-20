use std::{cmp, collections::{HashMap, HashSet}, fs, vec};

const grid_x: i32 = 101;
const grid_y: i32 = 103;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let robots = process_input(&input);
    println!("Part 1: {}", part1(&robots));
    println!("Part 2: {}", part2(&robots));

}

fn process_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    input.lines().map(|line|{
        let first_split = line.split(" ").collect::<Vec<&str>>();
        let left_split = first_split[0].split('=').skip(1).next().unwrap();
        let right_split = first_split[1].split('=').skip(1).next().unwrap();
        let p_split = left_split.split(",").map(|val| {val.parse().unwrap()}).collect::<Vec<i32>>();
        let v_split = right_split.split(",").map(|val| {val.parse().unwrap()}).collect::<Vec<i32>>();
        (p_split[0], p_split[1], v_split[0], v_split[1])
    }).collect()
}

fn predict_robot(x: i32, y:i32, vx: i32, vy:i32, time: i32) -> (i32, i32) {
    let mut final_x = (x + vx * time) % grid_x;
    if final_x < 0 { final_x += grid_x};
    let mut final_y = (y + vy * time) % grid_y;
    if final_y < 0 { final_y += grid_y};
    (final_x, final_y)
}

fn part1(robots: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let positions = robots.iter().map(|(x, y, vx, vy)| {predict_robot(*x, *y, *vx, *vy, 100)});
    let mut quadrants = vec![0; 4];
    for (x, y) in positions {
        let mut index: i32 = 0;
        if x == grid_x/2 {
            index = -10;
        } else if x > grid_x/2 {
            index += 2;
        }

        if y == grid_y/2 {
            index = -10;
        } else if y > grid_y/2 {
            index += 1;
        }

        if index >= 0 {
            quadrants[index as usize] += 1;
        }

    }
    println!("{:?}", quadrants);
    quadrants.iter().fold(1, |acc, val| {acc * val})
}

fn display_bots(positions: &Vec<(i32, i32)>){
    let mut grid = vec![vec![false; grid_x as usize]; grid_y as usize];
    for (x, y) in positions {
        grid[*y as usize][*x as usize] = true;
    }
    grid.iter().for_each(|row| {
        row.iter().for_each(|val|{
            match val {
                true => print!("*"),
                _ => print!(".")
            }
        });
        println!("");
    });
}

fn part2(robots: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let mut best = 0;
    let mut max_score = 0;
    for i in 0..10000 {
        let pos_set: HashSet<(i32, i32)> = robots.iter().map(|(x, y, vx, vy)| {predict_robot(*x, *y, *vx, *vy, i)}).collect();
        let mut score = 0;
        for (x, y) in pos_set.iter() {
            let mut count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {continue;}
                    if pos_set.contains(&(x+i, y+j)) {count += 1}
                }
            }
            score += cmp::max(2, count);
            if score > max_score {
                max_score = score;
                best = i;
            }
        }
    }
    let best_pos = robots.iter().map(|(x, y, vx, vy)| {predict_robot(*x, *y, *vx, *vy, best)}).collect::<Vec<_>>();
    display_bots(&best_pos);
    best
}