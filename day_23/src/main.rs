use core::panic;
use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        if let [node1, node2] = line.split("-").collect::<Vec<_>>().as_slice() {
            graph.entry(*node1).or_default().push(*node2);
            graph.entry(*node2).or_default().push(*node1);
        }else{
            panic!("Some Ting Wong");
        }
    });
    graph
}

fn part1(input: &str) -> usize{
    let graph = build_graph(input);
    let mut lans = HashSet::new();
    for (node, connections) in graph.iter() {
        if !node.starts_with('t') { continue; }
        for i in 0..connections.len()-1 {
            for j in i+1..connections.len() {
                let node1 = connections[i];
                let node2 = connections[j];
                if graph.get(node1).unwrap().contains(&node2) 
                && graph.get(node2).unwrap().contains(&node1){
                    let mut list = [node, node1, node2];
                    list.sort();
                    lans.insert(list.join(""));
                }
            }
        }
    }
    lans.len()
}

fn check_connectivity<'a>(prior: &mut HashSet<&'a str>, connections: &Vec<&'a str>, graph: &HashMap<&str, Vec<&'a str>>, i: usize) -> Vec<&'a str> {
    if i == connections.len() {
        return prior.clone().into_iter().collect();
    }
    if graph[connections[i]].iter().filter(|con| {prior.contains(*con)}).count() != prior.len()
    {
        return prior.clone().into_iter().collect();
    }

    let without = check_connectivity(prior, connections, graph, i+1);
    prior.insert(connections[i]);
    let with= check_connectivity(prior, connections, graph, i+1);
    prior.remove(connections[i]);

    if without.len() > with.len() {
        without
    }else{
        with
    }
}

fn part2(input: &str) -> String{
    let graph = build_graph(input);
    let mut largest = vec![];
    for (node, connections) in graph.iter() {
        let mut prior = HashSet::from([*node]);
        let res = check_connectivity(&mut prior, connections, &graph, 0);
        if res.len() > largest.len() {
            largest = res;
        }
    };
    largest.sort();
    largest.join(",")
}