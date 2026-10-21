use std::{collections::VecDeque, fs::read_to_string, vec};

const GRID_SIZE:usize = 71;
const BYTES:usize = 1024;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let grid = process_input(&input);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {:?}", part2(&input));

}

fn process_input(input: &str) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![true; GRID_SIZE]; GRID_SIZE];

    let mut count = 0;
    for line in input.lines() {
        if count >= BYTES{
            break
        }
        if let [x, y] = line.split(',').map(|c| {c.parse().unwrap()}).collect::<Vec<usize>>().as_slice() {
            grid[*y][*x] = false;
        }else{
            panic!("whoops");
        }
        count += 1;
    }
    println!("{}", count);
    grid
}

fn in_bound(i: i32, j: i32, grid: &Vec<Vec<bool>>) -> bool{
    i >= 0 && j >= 0 && i < grid.len() as i32 && j < grid[0].len() as i32
}


fn get_neighbours(i: usize, j: usize, grid: &Vec<Vec<bool>>, visited: &Vec<Vec<bool>>) -> Vec<(usize, usize)>{
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut neighbours = Vec::new();
    for (di, dj) in dirs {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if in_bound(ni, nj, grid) && grid[ni as usize][nj as usize] && !visited[ni as usize][nj as usize]{
            neighbours.push((ni as usize, nj as usize));
        }
    }
    neighbours
}

fn print_grid(grid: &Vec<Vec<bool>>){
    for row in grid {
        row.iter().for_each(|c| {print!("{}", match c {
            true => '.',
            false => '#',
        })});
        println!()
    }
}

fn part1(grid: &Vec<Vec<bool>>) -> usize{
    print_grid(grid);
    let mut visited = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    let mut q = VecDeque::new();
    q.push_back((0,0,0));
    while !q.is_empty(){
        let (i, j, d) = q.pop_front().unwrap();
        for (ni, nj) in get_neighbours(i, j, grid, &visited){
            visited[ni][nj] = true;
            if ni == GRID_SIZE-1 && nj == GRID_SIZE-1 {
                return d+1
            }
            q.push_back((ni, nj, d+1));
        }
    };
    return 0;
}

fn part2(input: &str) -> (usize, usize){
    let mut grid = vec![vec![true; GRID_SIZE]; GRID_SIZE];
    for line in input.lines() {
        let mut point = (0, 0);
        if let [x, y] = line.split(',').map(|c| {c.parse().unwrap()}).collect::<Vec<usize>>().as_slice() {
            grid[*y][*x] = false;
            point = (*x, *y);
        }
        let mut visited = vec![vec![false; GRID_SIZE]; GRID_SIZE];
        let mut q = VecDeque::new();
        q.push_back((0,0,0));
        let mut found = false;
        while !q.is_empty(){
            let (i, j, d) = q.pop_front().unwrap();
            for (ni, nj) in get_neighbours(i, j, &grid, &visited){
                visited[ni][nj] = true;
                if ni == GRID_SIZE-1 && nj == GRID_SIZE-1 {
                    found = true;
                    break
                }
                q.push_back((ni, nj, d+1));
            }
        };
        if !found{
            return point;
        }
        
    }
    (0,0)
}



