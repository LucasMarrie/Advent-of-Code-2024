use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = process_input(&input);
    println!("Part 1: {:}", part1(&map));
    println!("Part 2: {:}", part2(&map));

}

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line| line.chars().map(|c| {c.to_digit(10).unwrap()}).collect()).collect()
}

fn in_bounds(map: &Vec<Vec<u32>>, i: i32, j:i32) -> bool {
    i >= 0 && j >= 0 && i < map.len() as i32 && j < map[0].len() as i32
}

fn get_neighbours(map: &Vec<Vec<u32>>, i: usize, j:usize, val: u32) -> Vec<(usize, usize)> {
    let dirs: [(i32, i32); 4] = [(0,1), (1, 0), (-1, 0), (0, -1)];
    let mut result = vec![];
    for (di, dj) in dirs {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if in_bounds(map, ni, nj) && map[ni as usize][nj as usize] == val + 1{
            result.push((ni as usize, nj as usize));
        }
    }
    result
}

fn explore_path(map: &Vec<Vec<u32>>, i: usize, j:usize, val: u32, visited: &mut HashSet<(usize, usize)>) -> u32{
    if val == 9 {
        if visited.insert((i, j)) {
            return 1;
        }else{
            return 0;
        }
    }
    get_neighbours(map, i, j, val).iter().map(|(ni, nj)| {
        explore_path(map, *ni, *nj, val+1, visited)
    }).sum()
}

fn part1(map: &Vec<Vec<u32>>) -> u32{
    map.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, val)| {
            if *val == 0 {
                let mut visited = HashSet::new();
                explore_path(map, i, j, *val, &mut visited)
            }else{
                0
            }
        }).sum::<u32>()
    }).sum()
}

fn explore_path_unique(map: &Vec<Vec<u32>>, i: usize, j:usize, val: u32) -> u32{
    if val == 9 {
        return 1;
    }
    get_neighbours(map, i, j, val).iter().map(|(ni, nj)| {
        explore_path_unique(map, *ni, *nj, val+1)
    }).sum()
}

fn part2(map: &Vec<Vec<u32>>) -> u32{
    map.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, val)| {
            if *val == 0 {
                explore_path_unique(map, i, j, *val)
            }else{
                0
            }
        }).sum::<u32>()
    }).sum()
}