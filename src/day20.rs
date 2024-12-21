//use std::{fs::File, io::Read};
//
//struct Point {
//    x: usize,
//    y: usize,
//}
//
//fn get_initial_and_end_position(map: &Vec<&str>) -> (Point, Point) {
//    let mut start = Point { x: 0, y: 0 };
//    let mut end = Point { x: 0, y: 0 };
//    map.into_iter().enumerate().for_each(|(x, row)| {
//        row.chars().enumerate().for_each(|(y, cell)| {
//            if cell == 'S' {
//                start = Point { x, y };
//            }
//            if cell == 'E' {
//                end = Point { x, y };
//            }
//        })
//    });
//    (start, end)
//}
//
//pub fn problem1() {
//    let mut file = File::open("input/day20_test.txt").unwrap();
//    let mut contents = String::new();
//    file.read_to_string(&mut contents).unwrap();
//    let map: Vec<&str> = contents.split('\n').collect();
//    let (start_position, end_position) = get_initial_and_end_position(&map);
//    println!("{},{} -- {},{}", start_position.x, start_position.y, end_position.x, end_position)
//}
//