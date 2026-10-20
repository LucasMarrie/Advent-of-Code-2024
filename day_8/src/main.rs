use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (node_map, bound_i, bound_j) = process_input(&input);
    println!("part 1: {:}", part1(&node_map, bound_i, bound_j));
    println!("part 2: {:}", part2(&node_map, bound_i, bound_j));

}

fn process_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize, usize){
    let mut bound_i = 0;
    let mut bound_j = 0;
    let mut result: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, line) in input.lines().enumerate(){
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                result.entry(c).or_insert(Vec::new()).push((i, j));
            }
            bound_j = j;
        }
        bound_i = i;
    };
    (result, bound_i + 1, bound_j + 1)
}

fn in_bounds(pos: (i32, i32), bound_i: usize, bound_j: usize) -> bool{
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < bound_i as i32 && pos.1 < bound_j as i32
}

fn anti_node_pos(node1 : (usize, usize), node2 : (usize, usize) ) -> (i32, i32) {
    let node1 = (node1.0 as i32, node1.1 as i32);
    let node2 = (node2.0 as i32, node2.1 as i32);
    let diff = (node1.0 - node2.0, node1.1 - node2.1);
    (node1.0 + diff.0, node1.1 + diff.1)
}

fn part1(node_map : &HashMap<char, Vec<(usize, usize)>>, bound_i: usize, bound_j: usize) -> usize{
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for nodes in node_map.values(){
        for node1 in nodes {
            for node2 in nodes {
                if node1 == node2 {continue;}
                let antinode = anti_node_pos(*node1, *node2);
                if in_bounds(antinode, bound_i, bound_j) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn anti_node_all_pos(node1 : (usize, usize), node2 : (usize, usize), bound_i: usize, bound_j: usize) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    let node1 = (node1.0 as i32, node1.1 as i32);
    let node2 = (node2.0 as i32, node2.1 as i32);
    let diff = (node1.0 - node2.0, node1.1 - node2.1);
    let mut curr = node1;
    while in_bounds(curr, bound_i, bound_j) {
        antinodes.push(curr);
        curr = (curr.0 + diff.0, curr.1 + diff.1);
    }
    antinodes
}

fn part2(node_map : &HashMap<char, Vec<(usize, usize)>>, bound_i: usize, bound_j: usize) -> usize{
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for nodes in node_map.values(){
        for node1 in nodes {
            for node2 in nodes {
                if node1 == node2 {continue;}
                let antinode_list = anti_node_all_pos(*node1, *node2, bound_i, bound_j);
                for antinode in antinode_list {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}


