use std::{fs::File,io::Read};
use regex::Regex;

pub fn problem1() {
    let mut file = File::open("input/day3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let result : usize = re.captures_iter(&contents).map(|captures| captures[1].parse::<usize>().unwrap()*captures[2].parse::<usize>().unwrap() ).sum();
    println!("[{}]", result);
}

pub fn problem2() {
    let mut file = File::open("input/day3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = "do()".to_owned() + &contents + "don't()";
    let mul_matcher = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let dodont_matcher = Regex::new(r"do\(\)((?s).*?)don't\(\)").unwrap();
    let result: usize = dodont_matcher.captures_iter(&input).map(|captures| mul_matcher.captures_iter(&captures[1]).map(|captures| captures[1].parse::<usize>().unwrap()*captures[2].parse::<usize>().unwrap() ).sum::<usize>()).sum::<usize>();    
    println!("[{}]", result);
}