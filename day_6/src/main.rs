use std::{collections::{hash_map, HashSet}, fs, hash::Hash};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<i32>> = create_grid(&input);
    let start = find_start_pos(&grid).unwrap();

    println!("part 1: {:}", part1(start, &grid));
    println!("part 2: {:}", part2(start, &grid));

}

fn create_grid(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line.chars().map(|c| {
        match c {
            '#' => 1,
            '.' => 0,
            _ => 2 
        }
    }).collect()).collect()
}

fn find_start_pos(grid: &Vec<Vec<i32>>) -> Option<(i32, i32)>{
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            if value == 2 {
                return Some((row_idx as i32, col_idx as i32)); // Return the position when `2` is found
            }
        }
    }
    None
}

fn rotate_dir(dir: (i32, i32)) -> (i32, i32) {
    let (i, j) = dir;
    if i == -1 {
        (0, 1)
    } else if j == 1 {
        (1, 0)
    } else if i == 1 {
        (0, -1)
    }else {
        (-1, 0)
    }
}

fn in_bounds(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> bool {
    i >= 0 && j >= 0 && i < grid.len() as i32 && j < grid[0].len() as i32
}

fn print_grid(grid: &Vec<Vec<i32>>, objects: &HashSet<(i32, i32)>, represent: char){
    for (i, row) in grid.iter().enumerate() {
        for (j, n) in row.iter().enumerate(){
            if objects.contains(&(i as i32, j as i32)) {
                print!("{}", represent);
            }else{
                print!("{}",match n { 1 => '#', _ => '.',});
            }
        }
        println!();
    }
}

fn get_patrolled(start: (i32, i32), grid: &Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    let mut visited: HashSet<(i32, i32, (i32, i32))> = HashSet::new();
    let mut patrolled: HashSet<(i32, i32)> = HashSet::new();

    let (mut prev_i, mut prev_j) = start;
    let mut dir = (-1, 0);
    while visited.insert((prev_i, prev_j, dir)) {
        patrolled.insert((prev_i, prev_j));
        let mut next_i = prev_i + dir.0;
        let mut next_j = prev_j + dir.1;

        if !in_bounds(next_i, next_j, &grid){
            break;
        }
        while grid[next_i as usize][next_j as usize] == 1 {
            dir = rotate_dir(dir);
            next_i = prev_i + dir.0;
            next_j = prev_j + dir.1;
            if !in_bounds(next_i, next_j, &grid){
                break;
            }
        }
        prev_i = next_i;
        prev_j = next_j;
    }

    patrolled
}

fn part1(start: (i32, i32), grid: &Vec<Vec<i32>>) -> i32{
    get_patrolled(start, grid).len() as i32
}

fn enters_loop(i: i32, j:i32, dir: (i32, i32), obstacle: (i32, i32), grid: &Vec<Vec<i32>>, visited: &mut HashSet<(i32, i32, (i32, i32))>) -> bool {
    if !visited.insert((i, j, dir)){
        return true
    }
    
    let mut next_i = i + dir.0;
    let mut next_j = j + dir.1;
    let mut next_dir = dir;

    if !in_bounds(next_i, next_j, grid){
        return false;
    
    }
    while (next_i == obstacle.0 && next_j == obstacle.1) || grid[next_i as usize][next_j as usize] == 1 {
        next_dir = rotate_dir(next_dir);
        next_i = i + next_dir.0;
        next_j = j + next_dir.1;
    };
    
    enters_loop(next_i, next_j, next_dir, obstacle, grid, visited)
    
}

fn part2(start: (i32, i32), grid: &Vec<Vec<i32>>) -> i32 {
    get_patrolled(start, &grid).iter().filter(|(i, j)|{
        if *i == start.0 && *j == start.1 {return false}
        let mut visited: HashSet<(i32, i32, (i32, i32))> = HashSet::new();
        enters_loop(start.0, start.1, (-1, 0), (*i, *j), &grid, &mut visited)
    }).count() as i32
}