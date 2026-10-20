use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (link_map, value_map) = process_input(&input);
    part2(&link_map, &value_map);
    print_gate(&build_gate("z02", &link_map, &value_map), 0);
    println!("------------------------");
    print_gate(&build_gate("z03", &link_map, &value_map), 0);
    println!("------------------------");
    print_gate(&build_gate("z04", &link_map, &value_map), 0);
    println!("------------------------");


}

fn process_input(input: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, bool>){
    let mut reading_value = true;
    let mut value_map = HashMap::new();
    let mut link_map = HashMap::new();
    for line in input.lines() {
        if line.len() == 0 {
            reading_value = false;
        }else if reading_value{
            let parts = line.split(":").map(|part| {part.trim()}).collect::<Vec<_>>();
            value_map.insert(parts[0], parts[1] == "1");
        }else{
            let split1 = line.split("->").map(|part| {part.trim()}).collect::<Vec<_>>();
            link_map.insert(split1[1], split1[0].split(" ").collect());
        }
    }
    (link_map, value_map)
}

fn get_value<'a>(node: &'a str, link_map : &HashMap<&'a str, Vec<&'a str>>, value_map: &mut HashMap<&'a str, bool>) -> bool {
    if value_map.contains_key(&node) {
        return *value_map.get(&node).unwrap();
    };
    let link = link_map.get(&node).unwrap();
    let res;
    if link[1] == "AND" {
        res = get_value(link[0], link_map, value_map) && get_value(link[2], link_map, value_map);
    }else if link[1] == "OR" {
        res = get_value(link[0], link_map, value_map) || get_value(link[2], link_map, value_map);
    }else {
        res = get_value(link[0], link_map, value_map) != get_value(link[2], link_map, value_map);
    }
    value_map.insert(node, res);
    res
}

fn part1(link_map : &HashMap<&str, Vec<&str>>, value_map: &HashMap<&str, bool>) -> u64{
    let mut value_map = value_map.clone();
    let mut node_list = link_map.keys().collect::<Vec<_>>();
    node_list.sort_by(|a, b| b.cmp(a));
    let mut result = 0;
    for node in node_list {
        if !node.starts_with("z") {break;}
        result <<= 1;
        if get_value(node, link_map, &mut value_map) { result += 1}
    }
    result
}


#[derive(Debug)]
enum Gate<'a>{
    OR(Box<Gate<'a>>, Box<Gate<'a>>),
    AND(Box<Gate<'a>>, Box<Gate<'a>>),
    XOR(Box<Gate<'a>>, Box<Gate<'a>>),
    NODE(&'a str),
}


use Gate::*;


fn build_gate<'a>(node: &'a str, link_map : &HashMap<&'a str, Vec<&'a str>>, value_map: &HashMap<&'a str, bool>) -> Gate<'a>{
    if let Some(link) = link_map.get(&node) {
        let gate1 = Box::new(build_gate(link[0], link_map, value_map));
        let gate2 = Box::new(build_gate(link[2], link_map, value_map));
        match link[1] {
            "AND" => AND(gate1, gate2),
            "OR" => OR(gate1, gate2),
            _ => XOR(gate1, gate2),
        }
    }else{
        NODE(node)
    }
}

fn print_gate(gate: &Gate, depth: usize){
    let indent = (0..depth).map(|_| {"  "}).collect::<String>();
    match gate {
        OR(..) => println!("{}OR", indent),
        AND(..) => println!("{}AND", indent),
        XOR(..) => println!("{}XOR", indent),
        NODE(n) => println!("{}{}", indent, n),
    };
    match gate {
        OR(n1, n2) 
        | AND(n1, n2) 
        | XOR(n1, n2)
         => {
            print_gate(n1.as_ref(), depth+1);
            print_gate(n2.as_ref(), depth+1)},
        _ => {},
    }
}

fn verify_carry(gate: &Gate, num:usize) -> bool{
    if num == 1 {
        verify_xor_root(gate, num);
    }
    if let OR(n1, n2) = gate {
        for gates in [[n1, n2], [n2, n1]] {
            if verify_and_root(gates[0], num){
                return verify_and_result(gate, num);
            }
        }
    }
    false
}

fn correct_root(root1: &Gate, root2: &Gate, num: usize) -> bool{
    if let NODE(node1) = root1 {
        if let NODE(node2) = root2 {
            if node1[1..].parse::<usize>().unwrap() == num && node2[1..].parse::<usize>().unwrap() == num {
                return true
            }
        }
    } 
    false
}

fn verify_and_root(gate: &Gate, num: usize) -> bool{
    if let AND(root1, root2) = gate {
        return correct_root(root1, root2, num);
    }
    false
}

fn verify_xor_root(gate: &Gate, num: usize) -> bool{
    if let XOR(root1, root2) = gate {
        return correct_root(root1, root2, num);
    }
    false
}

fn verify_xor_result(gate: &Gate, num: usize) -> bool{
    if let XOR(n1, n2) = gate{
        for gates in [[n1, n2], [n2, n1]] {
            if verify_xor_root(gates[0], num){
                return verify_carry(gates[1], num-1);
            }
        }
    }
    false
}

fn verify_and_result(gate: &Gate, num: usize) -> bool{
    if let AND(n1, n2) = gate{
        for gates in [[n1, n2], [n2, n1]] {
            if verify_xor_root(gates[0], num){
                return verify_carry(gates[1], num-1);
            }
        }
    }
    false
}

fn part2(link_map : &HashMap<&str, Vec<&str>>, value_map: &HashMap<&str, bool>) {
    
    
}
