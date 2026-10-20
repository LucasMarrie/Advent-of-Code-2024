use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (rules, updates) = process_input(&input);
    println!("part 1: {:}", part_1(&rules,&updates));
    println!("part 2: {:}", part_2(&rules,&updates));

}

fn process_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>){
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut checking_rules = true;
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        if line.is_empty(){
            checking_rules = false;
            continue;
        }
        if checking_rules {
            let args = line.split('|').map(|s| {s.parse().unwrap()}).collect::<Vec<_>>();
            if let [key, value] = &args[..]{
                rules.entry(*key).or_insert(HashSet::new()).insert(*value);
            }
        }else{
            updates.push(line.split(',').map(|s| {s.parse().unwrap()}).collect());
        }
    }
    (rules, updates)
}

fn part_1(rules: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32{
    let mut total = 0;
    for update in updates{
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        rules.iter().for_each(|(key, ruleset)|{
            if update.contains(key){
                graph.insert(*key, update.iter().cloned().filter(|item| {ruleset.contains(item)}).collect());
            }
        });
        let sorted_list = top_sort(&graph);
        if *update == sorted_list {
            total += sorted_list[sorted_list.len()/2];
        }
    }
    total
}

fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, stack : &mut Vec<i32>){
    if visited.contains(&node) {return;}
    visited.insert(node);
    for nbr in graph.get(&node).unwrap_or(&vec![]) {
        dfs(*nbr, graph, visited, stack);
    };
    stack.push(node);
}

fn top_sort(graph: &HashMap<i32, Vec<i32>>) -> Vec<i32>{
    let mut visited: HashSet<i32> = HashSet::new();
    let mut stack: Vec<i32> = vec![];
    for (node, _) in graph {
        dfs(*node, graph, &mut visited, &mut stack);
    };
    stack.reverse();
    stack
}

fn part_2(rules: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32{
    let mut total = 0;
    for update in updates{
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        rules.iter().for_each(|(key, ruleset)|{
            if update.contains(key){
                graph.insert(*key, update.iter().cloned().filter(|item| {ruleset.contains(item)}).collect());
            }
        });
        let sorted_list = top_sort(&graph);
        if *update != sorted_list {
            total += sorted_list[sorted_list.len()/2];
        }
    }
    total
}