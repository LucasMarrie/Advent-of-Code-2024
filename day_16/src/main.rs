use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() { 
    let input = read_to_string("input.txt").unwrap();
    let grid = process_input(&input);
    let start = (grid.len() - 2, 1);
    let goal = (1, grid.len()-2);
    println!("Part 1: {}", part1(&grid, start, goal));
    println!("Part 2: {}", part2(&grid, start, goal));
}

fn process_input(input: &str) -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|c| {match c {
            '#' => false,
            _ => true,
        }}).collect());
    };
    grid
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1 ,0), (0, -1)];

fn part1(grid: &Vec<Vec<bool>>, start: (usize, usize), goal: (usize, usize)) -> usize{
    let mut visited = HashSet::new();
    let mut q = HashMap::new();
    q.insert((start, (0, 1)), 0);

    while !q.is_empty(){
        let item = q.iter().min_by_key(|&(_, v)| *v).unwrap();
        let ((node, d), cost) = q.remove_entry(&item.0.clone()).unwrap();
        if node.0 == goal.0 && node.1 == goal.1 {
            return cost;
        }
        visited.insert((node, d));
        for dir in DIRS {
            let ncost;
            if dir == d {
                ncost = cost + 1 
            }else{
                ncost = cost + 1000 + 1
            }
            let n_node = ((node.0 as i32 + dir.0) as usize, (node.1 as i32 + dir.1) as usize);
            if grid[n_node.0][n_node.1] && !visited.contains(&(n_node, dir)) && ncost < *q.get(&(n_node, dir)).unwrap_or(&100000000){
                q.insert((n_node, dir), ncost);
            }
        }
    };
    return 0;
}

fn trace_parents(node: (usize, usize), dir: (i32, i32), parents: &HashMap<((usize, usize), (i32, i32)), Vec<((usize, usize), (i32, i32))>>, included: &mut HashSet<(usize, usize)>) {
    included.insert(node);
    println!("{:?}", node);
    for parent in parents.get(&(node, dir)).unwrap_or(&vec![]) {
        trace_parents(parent.0, parent.1, parents, included);
    }
}

fn print_paths(grid: &Vec<Vec<bool>>, included: &HashSet<(usize, usize)>){
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            print!("{}", match c {
                true => if included.contains(&(i, j)) {'O'} else {'.'},
                false => '#'
            });
        }
        println!()
    }
}

fn part2(grid: &Vec<Vec<bool>>, start: (usize, usize), goal: (usize, usize)) -> usize{
    let mut visited = HashSet::new();
    let mut parents = HashMap::new();
    let mut q = HashMap::new();
    q.insert((start, (0, 1)), 0);

    while !q.is_empty(){
        let item = q.iter().min_by_key(|&(_, v)| *v).unwrap();
        let ((node, dir), cost) = q.remove_entry(&item.0.clone()).unwrap();
        if node.0 == goal.0 && node.1 == goal.1 {
            let mut included = HashSet::new();
            trace_parents(goal, dir, &parents, &mut included);
            q.iter().for_each(|((other_node, other_dir), other_cost) | {
                if *other_cost == cost && *other_node == goal {
                    trace_parents(*other_node, *other_dir, &parents, &mut included);
                }
            });
            print_paths(grid, &included);
            return included.len()
        }
        visited.insert((node, dir));
        for n_dir in DIRS {
            let ncost;
            if n_dir == dir {
                ncost = cost + 1 
            }else{
                ncost = cost + 1000 + 1
            }
            let n_node = ((node.0 as i32 + n_dir.0) as usize, (node.1 as i32 + n_dir.1) as usize);
            if grid[n_node.0][n_node.1] && !visited.contains(&(n_node, n_dir)){
                if ncost < *q.get(&(n_node, n_dir)).unwrap_or(&100000000){
                    q.insert((n_node, n_dir), ncost);
                    parents.insert((n_node, n_dir), vec![(node, dir)]);
                }else if ncost == *q.get(&(n_node, n_dir)).unwrap_or(&100000000){
                    parents.get_mut(&(n_node, n_dir)).unwrap().push((node, dir));
                }
            }
        }
    };
    0
}