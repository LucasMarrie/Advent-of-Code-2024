use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let drive = process_input(&input);
    println!("part 1: {:}", part_1(drive.clone()));
    println!("part 2: {:}", part_2(&drive));
}

fn process_input(input: &str) -> Vec<i32>{
    input.chars().map(|c| {c.to_digit(10).unwrap() as i32}).collect()
}

fn get_check_sum(start: i32, end: i32, id: i32) -> i128{
    (start..end).map(|num| {num * id} as i128).sum()
}

fn part_1(mut drive: Vec<i32>) -> i128{
    let mut sum: i128 = 0;
    let mut i = 0;
    let mut j = drive.len() - 1;
    let mut block_no = 0;
    while i <= j {
        if i % 2 == 0 {
            sum += get_check_sum(block_no, block_no + drive[i], i as i32 / 2);
            block_no += drive[i];
            i += 1;
        } else if j % 2 == 0 {
            if drive[i] < drive[j] {
                sum += get_check_sum(block_no, block_no + drive[i], j as i32 / 2);
                drive[j] -= drive[i];
                block_no += drive[i];
                i += 1;
            }else{
                sum += get_check_sum(block_no, block_no + drive[j], j as i32 / 2);
                drive[i] -= drive[j];
                block_no += drive[j];
                if drive[i] == 0 {
                    i += 1;
                }
                j -= 1;
            }
        } else {
            j -= 1;
        }

    }
    sum
}


fn allocate_space(drive: &Vec<i32>, allocations: &mut Vec<i32>, space: i32, end_i: usize) -> Option<i32>{
    let mut block_no = 0;
    for i in 0..end_i {
        if i % 2 == 1 && drive[i] - allocations[i] >= space{
            block_no += allocations[i];
            allocations[i] += space;
            return Some(block_no);
        }
        block_no += drive[i];
    }
    None
}

fn part_2(drive: &Vec<i32>) -> i128{
    let mut allocations: Vec<i32> = vec![0; drive.len()];
    let mut sum = 0;
    let mut block_no = drive.iter().sum();
    for j in (0..drive.len()).rev() {
        block_no -= drive[j];
        if j % 2 == 1 {continue;}
        if let Some(allocated_block) = allocate_space(drive, &mut allocations, drive[j], j) {
            sum += get_check_sum(allocated_block, allocated_block + drive[j], j as i32 / 2)
        }else{
            sum += get_check_sum(block_no, block_no + drive[j], j as i32 / 2)
        }
    }
    sum   
}
