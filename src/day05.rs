use std::{collections::{HashMap, HashSet}, fs::File, io::Read};

fn parse_ordering_rules(page_ordering_rules_str: &str) -> HashMap<u32, Vec<u32>> {
    let mut result = HashMap::new();
    page_ordering_rules_str.split("\n").for_each(|str| {
        let (first_str, second_str) = str.split_once("|").unwrap();
        let first  = first_str.parse::<u32>().unwrap();
        let second = second_str.parse::<u32>().unwrap();
        result.entry(second).or_insert_with(|| Vec::new()).push(first);
    });
    result
}

fn parse_update_numbers(page_update_numbers_str: &str) -> Vec<Vec<u32>> {
    page_update_numbers_str.split("\n").map(|s| s.split(",").map(|number| number.parse().unwrap()).collect()).collect()
}

fn is_line_valid(line: &Vec<u32>, ordering_rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut visited_nodes = HashSet::new();
    let nodes : HashSet<u32> = HashSet::from_iter(line.iter().cloned());
    for num in line {
        if let Some(deps) = ordering_rules.get(num) {
            if deps.into_iter().any(|dep: &u32| !visited_nodes.contains(dep) && nodes.contains(dep)) {
                return 0;
            }
        }
        visited_nodes.insert(num);
    }
    line[line.len()/2]
}

#[allow(dead_code)]
fn make_line_valid(line: &mut Vec<u32>, _ordering_rules: &HashMap<u32, Vec<u32>>) {
    let mut _idx = 0;
    let _nodes : HashSet<u32> = HashSet::from_iter(line.iter().cloned());
    while _idx != line.len() {
        
    }
}

pub fn problem1() {
    let mut file = File::open("input/day5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (page_ordering_rules_str, page_update_numbers_str) = contents.split_once("\n\n").unwrap();

    let page_ordering_rules : HashMap<u32, Vec<u32>> = parse_ordering_rules(page_ordering_rules_str);
    let update_numbers = parse_update_numbers(page_update_numbers_str);
    let result : u32 = update_numbers.into_iter().map(|line| is_line_valid(&line, &page_ordering_rules)).sum();
    println!("{}", result);
}