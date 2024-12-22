use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn parse_ordering_rules(i_page_ordering_rules_str: &str) -> HashMap<u32, Vec<u32>> {
    let mut result = HashMap::new();
    i_page_ordering_rules_str.split("\n").for_each(|str| {
        let (first_str, second_str) = str.split_once("|").unwrap();
        let first = first_str.parse::<u32>().unwrap();
        let second = second_str.parse::<u32>().unwrap();
        result
            .entry(second)
            .or_insert_with(|| Vec::new())
            .push(first);
    });
    result
}

fn parse_update_numbers(i_page_update_numbers_str: &str) -> Vec<Vec<u32>> {
    i_page_update_numbers_str
        .split("\n")
        .map(|s| s.split(",").map(|number| number.parse().unwrap()).collect())
        .collect()
}

fn is_line_valid(i_line: &Vec<u32>, i_ordering_rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut visited_nodes = HashSet::new();
    let nodes: HashSet<u32> = HashSet::from_iter(i_line.iter().cloned());
    for num in i_line {
        if let Some(deps) = i_ordering_rules.get(num) {
            if deps
                .into_iter()
                .any(|dep: &u32| !visited_nodes.contains(dep) && nodes.contains(dep))
            {
                return false;
            }
        }
        visited_nodes.insert(num);
    }
    true
}

fn compute_line_value(i_line: &Vec<u32>, i_ordering_rules: &HashMap<u32, Vec<u32>>) -> u32 {
    if !is_line_valid(i_line, i_ordering_rules) {
        return 0;
    }
    i_line[i_line.len() / 2]
}

#[allow(dead_code)]
fn make_line_valid(i_line: &mut Vec<u32>, i_ordering_rules: &HashMap<u32, Vec<u32>>) {
    let mut idx = 0;
    while idx != i_line.len() {
        let mut swapped_any = false;
        if let Some(deps) = i_ordering_rules.get(&i_line[idx]) {
            for dep in deps {
                if !i_line[0..idx].contains(dep) {
                    if let Some(other_node_idx) = i_line.iter().position(|num| num == dep) {
                        i_line.swap(idx, other_node_idx);
                        swapped_any = true;
                        break;
                    }
                }
            }
        }
        if !swapped_any {
            idx += 1;
        }
    }
}

fn compute_line_value_only_invalid(
    i_line: &Vec<u32>,
    i_ordering_rules: &HashMap<u32, Vec<u32>>,
) -> u32 {
    let mut line = i_line.clone();
    if is_line_valid(&line, i_ordering_rules) {
        return 0
    }
    make_line_valid(&mut line, i_ordering_rules);
    //println!("{:?}", line);
    return line[line.len() / 2]
}

pub fn problem1() {
    let mut file = File::open("input/day5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (page_ordering_rules_str, page_update_numbers_str) = contents.split_once("\n\n").unwrap();

    let page_ordering_rules: HashMap<u32, Vec<u32>> = parse_ordering_rules(page_ordering_rules_str);
    let update_numbers = parse_update_numbers(page_update_numbers_str);
    let result: u32 = update_numbers
        .into_iter()
        .map(|line| compute_line_value(&line, &page_ordering_rules))
        .sum();
    println!("{}", result);
}

pub fn problem2() {
    let mut file = File::open("input/day5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (page_ordering_rules_str, page_update_numbers_str) = contents.split_once("\n\n").unwrap();

    let page_ordering_rules: HashMap<u32, Vec<u32>> = parse_ordering_rules(page_ordering_rules_str);
    let update_numbers = parse_update_numbers(page_update_numbers_str);
    let result: u32 = update_numbers
        .into_iter()
        .map(|line| compute_line_value_only_invalid(&line, &page_ordering_rules))
        .sum();
    println!("{}", result);
}
