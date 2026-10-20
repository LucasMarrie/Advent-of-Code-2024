use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

}



fn process_input(input: &str) -> (Vec<Vec<u8>>, Vec<(i32, i32)>) {
    let mut reading_grid = true;
    let mut grid: Vec<Vec<u8>> = Vec::new(); 
    let mut instructions: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            reading_grid = false;
            continue;
        }
        if reading_grid {
            grid.push(line.chars().map(|c| {
                match c {
                    'O' => 1,
                    '#' => 3,
                    '@' => 4,
                    _ => 0,
                }
            }).collect());
        } else {
            instructions.append(&mut line.chars().map(|c|{
                match c {
                    '^' => (-1, 0),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => (0, 1)
                }
            }).collect());
        }
    };
    (grid, instructions)
}


fn get_robot_pos(grid: &Vec<Vec<u8>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0.. grid[i].len() {
            if grid[i][j] == 4 {
                return (i as i32, j as i32)
            }
        }
    }
    (-1, -1)
}

fn move_box(i:i32, j:i32, di:i32, dj:i32, grid: &mut Vec<Vec<u8>>) -> bool{
    match grid[i as usize][j as usize] {
        3 =>  false,
        0 =>  true,
        _ => if move_box(i + di, j + dj, di, dj, grid) {
                grid[(i+di) as usize][(j+dj) as usize] = grid[i as usize][j as usize];
                grid[i as usize][j as usize] = 0;
                true
            } else {
                false
            }
    }
}

fn part1(input: &str) -> usize{
    let (mut grid, instructions) = process_input(input);
    let (mut r_x, mut r_y) = get_robot_pos(&grid);
    for (di, dj) in instructions {
        if move_box(r_x, r_y, di, dj, &mut grid) {
            r_x += di;
            r_y += dj;
        }
    };
    grid.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().filter_map(|(j, val)| {
            if *val == 1 {
                Some(i * 100 + j)
            }else{
                None
            }
        }).sum::<usize>()
    }).sum()
}

fn process_input2(input: &str) -> (Vec<Vec<u8>>, Vec<(i32, i32)>) {
    let mut reading_grid = true;
    let mut grid: Vec<Vec<u8>> = Vec::new(); 
    let mut instructions: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            reading_grid = false;
            continue;
        }
        if reading_grid {
            let mut row = vec![];
            for c in line.chars() {
                match c {
                    'O' => {row.push(1); row.push(2);},
                    '#' => {row.push(3); row.push(3);},
                    '@' => {row.push(4); row.push(0);},
                    _ => {row.push(0); row.push(0);},
                }
            }
            grid.push(row);
        } else {
            instructions.append(&mut line.chars().map(|c|{
                match c {
                    '^' => (-1, 0),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => (0, 1)
                }
            }).collect());
        }
    };
    (grid, instructions)
}

fn can_move_recur(i:i32, j:i32, di:i32, dj:i32, grid: &Vec<Vec<u8>>, second_part: bool) -> bool {
    match grid[i as usize][j as usize] {
        3 => false,
        0 => true,
        1 => can_move_recur(i+di, j+dj, di, dj, grid, false) 
            && (di == 0 || second_part || can_move_recur(i, j+1, di, dj, grid, true)),
        2 => can_move_recur(i+di, j+dj, di, dj, grid, false) 
            && (di == 0 || second_part || can_move_recur(i, j-1, di, dj, grid, true)),
        _ => can_move_recur(i+di, j+dj, di, dj, grid, false) 
    }
}

fn move_big_box(i:i32, j:i32, di:i32, dj:i32, grid: &mut Vec<Vec<u8>>, second_part: bool) -> bool{
    if match grid[i as usize][j as usize] {
        3 => false,
        0 => true,
        1 => move_big_box(i+di, j+dj, di, dj, grid, false) 
            && (di == 0 || second_part || move_big_box(i, j+1, di, dj, grid, true)),
        2 => move_big_box(i+di, j+dj, di, dj, grid, false) 
            && (di == 0 || second_part || move_big_box(i, j-1, di, dj, grid, true)),
        _ => move_big_box(i+di, j+dj, di, dj, grid, false) 
    } {
        if grid[i as usize][j as usize] != 0 {
            grid[(i+di) as usize][(j+dj) as usize] = grid[i as usize][j as usize];
            grid[i as usize][j as usize] = 0;
        }
        true
    } else {
        false
    }
}

fn print_grid(grid: &Vec<Vec<u8>>){
    for row in grid {
        for val in row {
            print!("{}", match val {
                3 => '#',
                2 => ']',
                1 => '[',
                0 => '.',
                _ => '@',
            })
        }
        println!()
    }
}

fn part2(input: &str) -> usize{
    let (mut grid, instructions) = process_input2(input);
    print_grid(&grid);

    let (mut r_x, mut r_y) = get_robot_pos(&grid);
    for (di, dj) in instructions {
        if can_move_recur(r_x, r_y, di, dj, &grid, false){
            move_big_box(r_x, r_y, di, dj, &mut grid, false);
            r_x += di;
            r_y += dj;  
        }
    };
    print_grid(&grid);
    grid.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().filter_map(|(j, val)| {
            if *val == 1 {
                Some(i * 100 + j)
            }else{
                None
            }
        }).sum::<usize>()
    }).sum()
}