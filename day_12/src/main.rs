use std::{collections::HashMap, fs, usize};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid = process_input(&input);
    println!("Part 1: {:}", part1(&grid));
    println!("Part 2: {:}", part2(&grid));
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| {line.chars().collect()}).collect()
}

fn in_bound(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool{
    i >= 0 && j >= 0 && i < grid.len() as i32 && j < grid[0].len() as i32
}

fn get_neighbours(value: char, i: usize, j: usize, grid: &Vec<Vec<char>>) -> Vec<Option<(usize, usize)>>{
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut neighbours = Vec::new();
    for (di, dj) in dirs {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if in_bound(ni, nj, grid) && grid[ni as usize][nj as usize] == value{
            neighbours.push(Some((ni as usize, nj as usize)));
        }else{
            neighbours.push(None);
        }
    }
    neighbours
}

fn get_area_perimiter(value: char, i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> (usize, usize) {
    if visited[i][j]{
        return (0,0);
    }
    visited[i][j] = true;
    let neighbours = get_neighbours(value, i, j, grid);
    let mut perimiter = 4;
    let mut area = 1;
    for  neighbour in neighbours {
        if let Some((ni, nj)) = neighbour {
            let (p, a) = get_area_perimiter(value, ni, nj, grid, visited);
            perimiter = perimiter + p - 1;
            area += a;
        }
    }
    (perimiter, area)
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    grid.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().fold(0, |total, (j, c)| {
            let (p, a) = get_area_perimiter(*c, i, j, grid, &mut visited);
            total + p * a
        })
    }).sum()
}


fn get_area_sides(value: char, i: usize, j: usize, sides: &mut HashMap<(usize, usize), Vec<usize>>, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> usize {
    if visited[i][j]{
        return 0;
    }
    visited[i][j] = true;
    let neighbours = get_neighbours(value, i, j, grid);
    let mut area = 1;
    for  (side, neighbour) in neighbours.iter().enumerate() {
        if let Some((ni, nj)) = neighbour {
            area += get_area_sides(value, *ni, *nj, sides, grid, visited);
        }else{
            if side < 2 {
                sides.entry((side, i)).or_default().push(j);
            } else {
                sides.entry((side, j)).or_default().push(i);
            }
        }
    }
    area
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    grid.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().fold(0, |total, (j, c)| {
            let mut sides = HashMap::new();
            let area = get_area_sides(*c, i, j, &mut sides, grid, &mut visited);
            let mut side_count = 0;
            for side_cells in sides.values_mut() {
                side_cells.sort();
                side_count += side_cells.windows(2).filter(|w| {w[1] - w[0] != 1}).count() + 1
            }
            total + side_count * area
        })
    }).sum()
}