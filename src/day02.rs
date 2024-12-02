use std::{fs::File, io::Read};
use itertools::Itertools;

fn is_combination_safe(curr : &i32, next : &i32, is_ascendent : bool) -> bool {
    !((is_ascendent && curr >= next) || (!is_ascendent && curr <= next) || (curr-next).abs() > 3)
}

fn is_seq_safe(sequence: &Vec<i32>) -> bool {
    let is_ascendent = sequence[0] < sequence[1];
    sequence.into_iter().tuple_windows().all(|(curr,next)| is_combination_safe(curr, next, is_ascendent))
}

fn is_seq_safe_skipping(sequence: &Vec<i32>, pos_to_skip: usize) -> bool {
    let skipped_sequence: Vec<i32> = sequence.into_iter().enumerate().filter(|(i, _)| *i != pos_to_skip).map(|(_, v)| *v).collect();
    is_seq_safe(&skipped_sequence)
}

pub fn problem1() {
    let mut file = File::open("input/day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut safe_reports = 0;
    for line in contents.lines() {
        let parsed_input : Vec<i32> = line.split(" ").map(|input| {input.parse().unwrap()}).collect();
        if is_seq_safe(&parsed_input)
        {
            safe_reports += 1;
        }
    }
    println!("[{}]", safe_reports)
}

pub fn problem2() {
    let mut file = File::open("input/day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut safe_reports = 0;
    for line in contents.lines() {
        let parsed_input : Vec<i32> = line.split(" ").map(|input| {input.parse().unwrap()}).collect();
        let is_safe =  is_seq_safe(&parsed_input) ||
                             (0..parsed_input.len()).any(|pos| is_seq_safe_skipping(&parsed_input, pos));
        if is_safe
        {
            safe_reports += 1;
        }
    }
    println!("[{}]", safe_reports)
}