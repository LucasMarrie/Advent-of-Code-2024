use std::{cmp, collections::{HashMap, VecDeque}, fs::read_to_string, result};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let codes = input.lines().collect::<Vec<_>>();
    println!("Part 1: {}", part1(&codes));
    println!("Part 2: {}", part2(&codes));
}

const A_PRESS : (i32, i32) = (2, 0);
const A_NUM : (i32, i32) = (2, 3);

fn num_to_string(coord: (i32, i32)) -> &'static str {
    match coord {
        (0,0) => "7",
        (2,0) => "8",
        (1,0) => "9",
        (0,1) => "4",
        (1,1) => "5",
        (2,1) => "6",
        (0,2) => "1",
        (1,2) => "2",
        (2,2) => "3",
        (1,3) => "0",    
        (2,3) => "A ",   
        _ => "BREAK",  
    }
}

fn press_to_string(coord: (i32, i32)) -> &'static str {
        match coord {
            (1, 0) => "^",
            (2, 0) => "A",
            (0, 1) => "<",
            (1, 1) => "v",
            (2, 1) => ">",
            _ => "BREAK"
        }
}

fn num_to_coord(num: char) -> (i32, i32){
    match num {
        '7' => (0,0),
        '8' => (1,0),
        '9' => (2,0),
        '4' => (0,1),
        '5' => (1,1),
        '6' => (2,1),
        '1' => (0,2),
        '2' => (1,2),
        '3' => (2,2),
        '0' => (1,3),    
        'A' => (2,3),   
        _ => (0, 0)    
    }
}

fn dir_to_arrow(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 1),
        (1, 0) => (2, 1),
        (0, -1) => (1, 0),
        _ => (10, 10)
    }
}

fn get_paths(from: (i32,i32), to: (i32, i32), isnumpad: bool) -> Vec<Vec<(i32, i32)>> {
    let mut results = Vec::new();
    let mut q = vec![(from, vec![])];
    while !q.is_empty() {
        let mut next_q = Vec::new();
        for (node, mut path) in q.into_iter() {
            if node == to {
                path.push(A_PRESS);
                results.push(path);
            }else{
                let dir_x = if to.0 - node.0 <= -1 {-1} else if to.0 - node.0 >= 1 {1} else {0};
                let dir_y = if to.1 - node.1 <= -1 {-1} else if to.1 - node.1 >= 1 {1} else {0};
                if dir_x != 0 {
                    let n_node = (node.0 + dir_x, node.1);
                    if isnumpad && num_to_string(n_node) != "BREAK" || !isnumpad && press_to_string(n_node) != "BREAK"{
                        let mut new_path = path.clone();
                        new_path.push(dir_to_arrow((dir_x, 0)));
                        next_q.push((n_node, new_path));
                    }
                }
                if dir_y != 0 {
                    let n_node = (node.0, node.1 + dir_y);
                    if isnumpad && num_to_string(n_node) != "BREAK" || !isnumpad && press_to_string(n_node) != "BREAK"{
                        path.push(dir_to_arrow((0, dir_y)));
                        next_q.push((n_node, path));
                    }
                }
            }
        }
        q = next_q;
    }
    results
}

fn list_to_string(input: &Vec<(i32, i32)>) -> String {
    input.iter().map(|c| {press_to_string(*c)}).collect()
}

fn shortest_length(input: Vec<(i32, i32)>, num_pad:usize, is_num_pad: bool, memo: &mut HashMap<(Vec<(i32, i32)>, usize), usize>) -> usize {
    let key = (input.clone(), num_pad);
    if num_pad == 0 {
        return input.len(); 
    }
    if memo.contains_key(&key){
        return *memo.get(&key).unwrap();
    }
    
    let mut from = if is_num_pad {A_NUM} else {A_PRESS};
    let mut total = 0;
    for node in input {
        let mut best = usize::MAX;
        for path in get_paths(from, node, is_num_pad){
            best = cmp::min(best, shortest_length(path, num_pad-1, false, memo));
        }
        total += best;
        from = node;
    }
    memo.insert(key, total);
    total
}

fn part1(codes: &Vec<&str>) -> usize{
    let mut result = 0;
    let mut memo = HashMap::new();
    for code in codes {
        let code_nodes: Vec<(i32, i32)> = code.chars().map(|c| {num_to_coord(c)}).collect();
        let length = shortest_length(code_nodes,3, true, &mut memo);
        println!();
        println!("{} length: {}", code, length);
        result += code.split("A").next().unwrap().parse::<usize>().unwrap() * length;
    }
    result
}


fn part2(codes: &Vec<&str>) -> usize{
    let mut result = 0;
    let mut memo = HashMap::new();
    for code in codes {
        let code_nodes = code.chars().map(|c| {num_to_coord(c)}).collect();
        result += code.split("A").next().unwrap().parse::<usize>().unwrap() * shortest_length(code_nodes,26, true, &mut memo);
    }
    result
}
