
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let list = process_input(&input);
    println!("part 1: {:}", part_1(&list));
    println!("part 2: {:}", part_2(&list));

}

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| {
        line
            .split(' ')
            .map(|num| num.parse::<i32>().unwrap())
            .collect()
        }).collect()
}

fn is_increasing(n1: i32, n2: i32) -> bool{
    n1 < n2 && n2 - n1 <= 3
}

fn is_decreasing(n1: i32, n2: i32) -> bool{
    n1 > n2 && n1 - n2 <= 3
}

fn part1_condition(list: &Vec<i32>) -> bool{
    let (inc, dec) = list.windows(2)
        .fold((true, true), |(increasing, decreasing), w| {
            (increasing && is_increasing(w[0], w[1]), decreasing && is_decreasing(w[0], w[1]))
    });
    inc || dec
}

fn part_1(list : &Vec<Vec<i32>>) -> i32 {
    list.iter().map(|l| part1_condition(l) as i32).sum()
}

fn part2_condition(list: &Vec<i32>) -> bool{
    let inc_func: fn(i32, i32) -> bool = is_increasing;
    let dec_func: fn(i32, i32) -> bool = is_decreasing;
    for func in [inc_func, dec_func] {
        if match list.windows(2).enumerate().filter(|(_i, w)| {
            !func(w[0], w[1])
        }).collect::<Vec<_>>().as_slice() {
            [(index1, _), (index2, _)] => index2 - index1 == 1 
                && func(list[*index1], list[index2+1]),
            [(index, _)] => *index == 0 || *index == list.len()-2 
                || func(list[index-1],list[index+1]) || func(list[*index], list[index+2]),
            [] => true,
            _ => false
        } {
            return true;
        }
    }
    return false;
    
}


fn part_2(list: &Vec<Vec<i32>>) -> i32 { 
    list.iter().map(|l| part2_condition(l) as i32).sum()
}
