use std::{collections::HashMap, collections::HashSet, fs::File, io::Read};

use itertools::Itertools;

fn mix(num1: u64, num2: u64) -> u64 {
    num1 ^ num2
}

fn prune(num: u64) -> u64 {
    num % 16777216
}

fn compute_secret_number(i_secret_number: u64, iterations: u64) -> usize {
    let mut result = i_secret_number;
    for _ in 0..iterations {
        let first_step = prune(mix(result, result << 6));
        let second_step = prune(mix(first_step, first_step >> 5));
        result = prune(mix(second_step, second_step << 11));
    }
    result as usize
}

fn compute_secret_number_with_delta(i_secret_number: u64) -> (usize, i32) {
    let new_number = compute_secret_number(i_secret_number, 1);
    (
        new_number,
        (new_number % 10) as i32 - (i_secret_number % 10) as i32,
    )
}

fn fill_sequences(
    i_secret_number: u64,
    o_sequences: &mut HashMap<(i32, i32, i32, i32), [usize; 2018]>,
    i_trade_number: usize,
) {
    let mut seen_sequences: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut secret_number: u64 = i_secret_number;
    (0..2000)
        .map(|_| {
            let (number, delta) = compute_secret_number_with_delta(secret_number);
            secret_number = number as u64;
            (number, delta)
        })
        .into_iter()
        .tuple_windows()
        .for_each(|((_, d1), (_, d2), (_, d3), (num, d4))| {
            let sequence = (d1, d2, d3, d4);
            if !seen_sequences.contains(&sequence) {
                seen_sequences.insert(sequence);
                let trades_for_sequence = o_sequences.entry(sequence).or_insert([0; 2018]);
                trades_for_sequence[i_trade_number] = num % 10;
            }
        });
}

pub fn problem1() {
    let mut file = File::open("input/day22.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result: usize = contents
        .lines()
        .map(|number_str| number_str.parse().unwrap())
        .map(|number| compute_secret_number(number, 2000))
        .sum::<usize>();

    println!("[{}]", result);
}

pub fn problem2() {
    let mut file = File::open("input/day22.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut sequences: HashMap<(i32, i32, i32, i32), [usize; 2018]> = HashMap::new();

    contents
        .lines()
        .map(|number_str| number_str.parse().unwrap())
        .enumerate()
        .for_each(|(trade_number, number)| fill_sequences(number, &mut sequences, trade_number));

    let result: usize = sequences
        .into_iter()
        .map(|(_, quant)| quant.into_iter().sum())
        .max()
        .unwrap();
    println!("[{}]", result);
}
