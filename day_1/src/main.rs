use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (list1, list2) = get_lists(&input);
    println!("part 1: {:}", part1(&list1, &list2));
    println!("part 2: {:}", part2(&list1, &list2));
}

fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = input
    .lines()                             // Split the string into lines
    .map(|line| { 
        line
            .split("   ")                      // Split by three spaces
            .map(|num| num.parse::<i32>().unwrap())     // Parse each part into an i32
            .collect::<Vec<_>>()       // Collect into a Vec
            .try_into()                // Convert Vec into [T; 2]
            .map(|[a, b]: [i32; 2]| (a, b))
            .unwrap()
        })
        .unzip();  
// and_then(|vec| (vec[0], vec[1])) // Return the tuple if the parsing was successful

    list1.sort();
    list2.sort();
    (list1, list2)
}

fn part1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    //get the sum of difference between the elements of the two lists
    list1.iter().zip(list2.iter()).map(|(a, b)| (a-b).abs()).sum()
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    //put elements of list2 into a frequency map all in one line
    let freq_map = list2.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    //return the sum of the product of list 1 elements and their frequency in list 2
    list1.iter().map(|x| x * freq_map.get(x).unwrap_or(&0)).sum()
}


