use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn day_1_1() -> i32 {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        sum += num / 3 - 2;
    }
    sum
}

fn day_1_2() -> i32 {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let mut num = line.unwrap().parse::<i32>().unwrap();
        loop {
            num = num / 3 - 2;
            if num < 1 {
                break;
            }
            sum += num;
        }
    }
    sum
}

fn day_2_inner(mut content: Vec<i32>, first: i32, second: i32) -> Option<i32> {
    content[1] = first;
    content[2] = second;
    let mut index = 0;
    while content[index] != 99 {
        let res_index = content[index + 3] as usize;
        let first = content[content[index + 2] as usize];
        let second = content[content[index + 1] as usize];
        content[res_index] = match content[index] {
            1 => first + second,
            2 => first * second,
            _ => return None,
        };
        index += 4;
    }
    Some(content[0])
}

fn day_2_1() -> i32 {
    let file = fs::read_to_string("inputs/day2.txt").unwrap();
    let content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    day_2_inner(content, 12, 2).unwrap()
}

fn day_2_2() -> i32 {
    let file = fs::read_to_string("inputs/day2.txt").unwrap();
    let content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..99 {
        for j in 0..99 {
            if day_2_inner(content.clone(), i, j) == Some(19_690_720) {
                return 100 * i + j;
            }
        }
    }
    0
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn day_3_inner() -> (i32, i32) {
    let file = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines: Vec<&str> = file.split("\r\n").collect();
    let mut wire_1 = HashMap::new();
    let mut current = Point { x: 0, y: 0 };
    let mut step = 0;
    for direction in lines[0].split(',').collect::<Vec<&str>>() {
        let num = direction[1..].parse::<i32>().unwrap();
        for _ in 0..num {
            match direction.get(0..1).unwrap() {
                "D" => current.y -= 1,
                "L" => current.x -= 1,
                "R" => current.x += 1,
                "U" => current.y += 1,
                _ => panic!("Unknown direction"),
            };
            step += 1;
            wire_1.insert(current, step);
        }
    }
    let mut closest = std::i32::MAX;
    let mut fewest = std::i32::MAX;
    current = Point { x: 0, y: 0 };
    step = 0;
    for direction in lines[1].split(',').collect::<Vec<&str>>() {
        let num = direction[1..].parse::<i32>().unwrap();
        for _ in 0..num {
            match direction.get(0..1).unwrap() {
                "D" => current.y -= 1,
                "L" => current.x -= 1,
                "R" => current.x += 1,
                "U" => current.y += 1,
                _ => panic!("Unknown direction"),
            };
            step += 1;
            if wire_1.contains_key(&current) {
                let diff = current.x.abs() + current.y.abs();
                if diff < closest {
                    closest = diff;
                }
                let sum = wire_1[&current] + step;
                if sum < fewest {
                    fewest = sum;
                }
            }
        }
    }
    (closest, fewest)
}

fn day_3_1() -> i32 {
    day_3_inner().0
}

fn day_3_2() -> i32 {
    day_3_inner().1
}

fn main() {
    assert_eq!(day_1_1(), 3_305_301);
    assert_eq!(day_1_2(), 4_955_106);
    assert_eq!(day_2_1(), 7_210_630);
    assert_eq!(day_2_2(), 3_892);
    assert_eq!(day_3_1(), 1_264);
    assert_eq!(day_3_2(), 37_390);
}
