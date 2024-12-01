use std::{collections::HashMap, fs::File, io::Read, iter::zip};

pub fn problem1() {
    let mut file = File::open("input/day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let split_result: Vec<i32> = line.split("   ").map(|input| {input.parse().unwrap()}).collect();
        left_list.push(split_result[0]);
        right_list.push(split_result[1]);
    }
    left_list.sort();
    right_list.sort();
    let result = zip(left_list, right_list).map(|(n1,n2)| (n1-n2).abs()).sum::<i32>();
    println!("[{}]", result);
}

pub fn problem2() {
    let mut file = File::open("input/day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut left_list_counter: HashMap<i32, i32> = HashMap::new();
    let mut right_list_counter: HashMap<i32, i32> = HashMap::new();
    for line in contents.lines() {
        let split_result: Vec<i32> = line.split("   ").map(|input| {input.parse().unwrap()}).collect();
        *left_list_counter.entry(split_result[0]).or_insert_with(|| 0) += 1;
        *right_list_counter.entry(split_result[1]).or_insert_with(|| 0) += 1;
    }
    let result = left_list_counter.into_iter().map(|(value, appearances_in_left_list)| {
        if let Some(appearances_in_right_list) = right_list_counter.get(&value) {
            return value * appearances_in_left_list * appearances_in_right_list;
        }
        0
    }).sum::<i32>();
    println!("[{}]", result);
}