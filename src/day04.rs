use std::{fs::File,io::Read};

pub fn is_valid_position(map: &Vec<Vec<char>>, (x,y): (i64,i64)) -> bool
{
    x >= 0 && y >= 0 && x < (map.len() as i64) && y < (map.len() as i64)
}

pub fn count_xmas_appearances(map: &Vec<Vec<char>>, (x,y): (i64, i64)) -> usize {
    let word_to_search : Vec<char> = "XMAS".chars().collect();
    if map[x as usize][y as usize] != word_to_search[0]
    {
        return 0;
    }

    let directions : Vec<(i64,i64)> = vec!((1,0),(0,1),(-1,0),(0,-1),(1,1),(-1,1),(1,-1),(-1,-1));
    let mut result = 0;
    for (mov_x,mov_y) in directions {
        let mut idx = 1;
        let (mut new_x, mut new_y) = (x+mov_x, y+mov_y);
        let mut is_valid = true;
        while idx < word_to_search.len() {
            if !is_valid_position(map, (new_x, new_y)){
                is_valid = false;
                break;
            }
            if map[new_x as usize][new_y as usize] != word_to_search[idx] {
                is_valid = false;
                break;
            }
            new_x += mov_x;
            new_y += mov_y;
            idx += 1;
        }
        if is_valid {
            result += 1;
        }
    }
    result
}

pub fn count_x_mas_appearances(map: &Vec<Vec<char>>, (x,y): (i64, i64)) -> usize {
    if map[x as usize][y as usize] != 'A'
    {
        return 0;
    }

    let directions : Vec<(i64,i64)> = vec!((1,1),(-1,1),(1,-1),(-1,-1));
    let mut m_count = 0;
    let mut s_count = 0;
    for (mov_x,mov_y) in directions {
        let (new_x, new_y) = (x+mov_x, y+mov_y);
        if !is_valid_position(map, (new_x,new_y)) {
            continue;
        }
        if map[new_x as usize][new_y as usize] == 'M' {
            m_count += 1;
        }
        if map[new_x as usize][new_y as usize] == 'S' {
            s_count += 1;
        }
    }
    if m_count == 2 && s_count == 2 && map[x as usize -1][y as usize -1] != map[x as usize +1][y as usize +1] {1} else {0}
}

pub fn problem1() {
    let mut file = File::open("input/day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let map : Vec<Vec<char>> = contents.split("\n").map(|line| line.chars().collect()).collect();
    let result : usize = map.clone().into_iter().enumerate().map(|(x,line)| line.into_iter().enumerate().map(|(y,_)| count_xmas_appearances(&map,(x as i64,y as i64))).sum::<usize>()).sum();
    println!("[{}]", result);
}

pub fn problem2() {
    let mut file = File::open("input/day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let map : Vec<Vec<char>> = contents.split("\n").map(|line| line.chars().collect()).collect();
    let result : usize = map.clone().into_iter().enumerate().map(|(x,line)| line.into_iter().enumerate().map(|(y,_)| count_x_mas_appearances(&map,(x as i64,y as i64))).sum::<usize>()).sum();
    println!("[{}]", result);
}