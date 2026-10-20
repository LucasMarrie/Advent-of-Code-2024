use std::{collections::VecDeque, fs::read_to_string};

const MIN_LEN: usize = 100;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (grid, start, end) = process_input(&input);
    println!("Part 1: {}", part1(&grid, start, end));
    println!("Part 2: {}", part2(&grid, start, end));
}

fn process_input(input: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)){
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => false,
                'S' => {start = (i, j); true},
                'E' => {end = (i, j); true},
                _ => true
            });
        }
        grid.push(row);
    };
    (grid, start, end)
}

fn get_neighbours(i: usize, j: usize, grid: &Vec<Vec<bool>>, visited: &Vec<Vec<bool>>) -> Vec<(usize, usize)>{
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut neighbours = Vec::new();
    for (di, dj) in dirs {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if grid[ni as usize][nj as usize] && !visited[ni as usize][nj as usize]{
            neighbours.push((ni as usize, nj as usize));
        }
    }
    neighbours
}

fn get_path_length(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    q.push_back((1, start));
    while let Some((cost, node)) = q.pop_front() {
        visited[node.0][node.1] = true;
        if node == end {
            return cost
        }
        for n_node in get_neighbours(node.0, node.1, grid, &visited){
            q.push_back((cost + 1, n_node));
        }
    }
    0
}

fn create_d_map(grid: &Vec<Vec<bool>>, end: (usize, usize)) -> Vec<Vec<usize>> {
    let mut q = VecDeque::new();
    let mut d_map = vec![vec![0; grid[0].len()]; grid.len()];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    q.push_back((0, end));

    while let Some((cost, node)) = q.pop_front() {
        d_map[node.0][node.1] = cost;
        visited[node.0][node.1] = true;
        for n_node in get_neighbours(node.0, node.1, grid, &visited){
            q.push_back((cost + 1, n_node));
        }
    }
    d_map
}

fn successful_cheats(grid: &Vec<Vec<bool>>, d_map_start: &Vec<Vec<usize>>, d_map_end: &Vec<Vec<usize>>, node: (usize, usize), radius: i32, original_dist: usize) -> usize {
    let mut count = 0;
    for i in -radius..=radius {
        for j in -radius..=radius {
            if i.abs() + j.abs() > radius { continue; }
            let n_node = (node.0 as i32 + i, node.1 as i32 + j);
            if n_node.0 < 0 || n_node.0 >= grid.len() as i32 || n_node.1 < 0 || n_node.1 >= grid[0].len() as i32
                || !grid[n_node.0 as usize][n_node.1 as usize] {continue;}
            let dist = d_map_start[node.0][node.1] + d_map_end[n_node.0 as usize][n_node.1 as usize] + i.abs() as usize + j.abs() as usize;
            if original_dist as i32 - dist as i32 >= MIN_LEN as i32 { count += 1}
        }
    }
    count 
}

fn part1(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let d_map_end = create_d_map(grid, end);
    let d_map_start = create_d_map(grid, start);
    let original_dist = get_path_length(grid, start, end);
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c {
                count += successful_cheats(grid, &d_map_start, &d_map_end, (i, j), 2, original_dist);
            }  
        }
    }
    count
}

fn print_grid(grid: &Vec<Vec<usize>>){
    for row in grid {
        for c in row {
            print!("{:0>2} ", c);
        }
        println!();
    }
}

fn part2(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> usize{
    let d_map_end = create_d_map(grid, end);
    let d_map_start = create_d_map(grid, start);
    let original_dist = get_path_length(grid, start, end);
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c {
                count += successful_cheats(grid, &d_map_start, &d_map_end, (i, j), 20, original_dist);
            }  
        }
    }
    count
}