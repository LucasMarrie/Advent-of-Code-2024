use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {:}", part1(&input));
    println!("Part 2: {:}", part2(&input));

}


fn part1(input: &str) -> usize {
    // Parse the input string into a grid of characters
    let grid: Vec<Vec<char>> = input
        .lines() // Split the input into rows
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();

    let target = "XMAS";
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Diagonal Down-Right
        (-1, 1),  // Diagonal Up-Right
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Diagonal Up-Left
        (1, -1),  // Diagonal Down-Left
    ];

    let target_chars: Vec<char> = target.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                if check_direction(&grid, &target_chars, row as isize, col as isize, dr, dc) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_direction(
    grid: &[Vec<char>],
    target_chars: &[char],
    start_row: isize,
    start_col: isize,
    dr: isize,
    dc: isize,
) -> bool {
    for (i, &ch) in target_chars.iter().enumerate() {
        let new_row = start_row + dr * i as isize;
        let new_col = start_col + dc * i as isize;

        if new_row < 0
            || new_row >= grid.len() as isize
            || new_col < 0
            || new_col >= grid[0].len() as isize
            || grid[new_row as usize][new_col as usize] != ch
        {
            return false;
        }
    }
    true
}

fn part2(input: &str) -> i32{
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    for i in 1..grid.len()-1{
        for j in 1..grid[i].len()-1{
            if grid[i][j] == 'A' {

                let offets  = [(i + 1, j + 1), (i -1, j + 1), (i-1, j-1), (i+1, j-1)];
                let ms: Vec<_> = offets.iter().filter(|(x, y)| {grid[*x][*y] == 'M'}).collect();
                let ss: Vec<_> = offets.iter().filter(|(x, y)| {grid[*x][*y] == 'S'}).collect();
                if ms.len() == 2 && ss.len() == 2 && ms.iter().all(|(x, y)| {ms[0].0 == *x || ms[0].1 == *y}){
                    total += 1;
                }

            }
        }
    }
    total
}