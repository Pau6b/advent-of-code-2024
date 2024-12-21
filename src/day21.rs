use lazy_static::lazy_static;
use std::{collections::HashMap, fs::File, io::Read};

fn convert_into_number(sequence: &str) -> usize {
    sequence
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap()
}

const INVALID_NUMPAD_POSITION: (u32, u32) = (0, 0);
const INVALID_DIRECTIONAL_POSITION: (u32, u32) = (1, 0);

lazy_static! {
    static ref NUMPAD_POSITIONS: HashMap<char, (u32, u32)> = {
        HashMap::from([
            ('7', (3, 0)),
            ('8', (3, 1)),
            ('9', (3, 2)),
            ('4', (2, 0)),
            ('5', (2, 1)),
            ('6', (2, 2)),
            ('1', (1, 0)),
            ('2', (1, 1)),
            ('3', (1, 2)),
            ('0', (0, 1)),
            ('A', (0, 2)),
        ])
    };
    static ref DIRECTIONAL_POSITIONS: HashMap<char, (u32, u32)> = {
        HashMap::from([
            ('^', (1, 1)),
            ('A', (1, 2)),
            ('<', (0, 0)),
            ('v', (0, 1)),
            ('>', (0, 2)),
        ])
    };
}

fn generate_sub_sequences(
    i_sequence: &str,
    i_times_appeared: usize,
    i_position_map: &HashMap<char, (u32, u32)>,
    i_invalid_position: (u32, u32),
    o_subsequences: &mut HashMap<String, usize>,
) {
    let (mut current_x, mut current_y) = i_position_map.get(&'A').unwrap();
    for c in i_sequence.chars() {
        let mut new_sequence = String::new();
        let (new_x, new_y) = i_position_map.get(&c).unwrap();
        while *new_x != current_x || *new_y != current_y {
            if *new_y < current_y && (current_x, *new_y) != i_invalid_position {
                new_sequence.push_str(&"<".repeat((current_y - new_y) as usize));
                current_y = *new_y;
                continue;
            }
            if *new_x < current_x && (*new_x, current_y) != i_invalid_position {
                new_sequence.push_str(&"v".repeat((current_x - new_x) as usize));
                current_x = *new_x;
                continue;
            }
            if *new_x > current_x && (*new_x, current_y) != i_invalid_position {
                new_sequence.push_str(&"^".repeat((new_x - current_x) as usize));
                current_x = *new_x;
                continue;
            }
            if *new_y > current_y && (current_x, *new_y) != i_invalid_position {
                new_sequence.push_str(&">".repeat((new_y - current_y) as usize));
                current_y = *new_y;
                continue;
            }
        }
        new_sequence.push('A');
        *o_subsequences.entry(new_sequence).or_insert_with(|| 0) += i_times_appeared;
    }
}

fn compute_complexity_directional(
    i_sequence: &HashMap<String, usize>,
    i_target_depth: u32,
    i_current_depth: u32,
) -> usize {
    if i_target_depth == i_current_depth {
        return i_sequence
            .into_iter()
            .map(|(str, quantity)| str.len() * quantity)
            .sum();
    }
    let mut next_level_sequences: HashMap<String, usize> = HashMap::new();
    i_sequence
        .into_iter()
        .for_each(|(sequence, times_appeared)| {
            generate_sub_sequences(
                &sequence,
                *times_appeared,
                &DIRECTIONAL_POSITIONS,
                INVALID_DIRECTIONAL_POSITION,
                &mut next_level_sequences,
            );
        });
    compute_complexity_directional(&next_level_sequences, i_target_depth, i_current_depth + 1)
}

fn compute_complexity(i_sequence: &str, i_target_depth: u32, i_current_depth: u32) -> usize {
    if i_target_depth == i_current_depth {
        return i_sequence.len();
    }
    let mut next_level_sequences: HashMap<String, usize> = HashMap::new();
    generate_sub_sequences(
        &i_sequence,
        1,
        &NUMPAD_POSITIONS,
        INVALID_NUMPAD_POSITION,
        &mut next_level_sequences,
    );
    compute_complexity_directional(&next_level_sequences, i_target_depth, i_current_depth + 1)
}

pub fn problem1() {
    let mut file = File::open("input/day21.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let sequences: Vec<&str> = contents.lines().collect();
    let results: Vec<usize> = sequences
        .into_iter()
        .map(|sequence| {
            let number = convert_into_number(sequence);
            let complexity = compute_complexity(sequence, 3, 0);
            println!(
                "sequence: [{}], number [{}], complexity [{}], result [{}]",
                sequence,
                number,
                complexity,
                number * complexity
            );
            number * complexity
        })
        .collect();
    println!("{:?}", results.into_iter().sum::<usize>());
}

pub fn problem2() {
    let mut file = File::open("input/day21.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let sequences: Vec<&str> = contents.lines().collect();
    let results: Vec<usize> = sequences
        .into_iter()
        .map(|sequence| {
            let number = convert_into_number(sequence);
            let complexity = compute_complexity(sequence, 26, 0);
            println!(
                "sequence: [{}], number [{}], complexity [{}], result [{}]",
                sequence,
                number,
                complexity,
                number * complexity
            );
            number * complexity
        })
        .collect();
    println!("{:?}", results.into_iter().sum::<usize>());
}
