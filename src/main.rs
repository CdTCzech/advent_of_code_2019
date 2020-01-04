use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn day_1_1() -> i32 {
    let file = File::open("inputs\\day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        sum += num / 3 - 2;
    }
    sum
}

fn day_1_2() -> i32 {
    let file = File::open("inputs\\day1.txt").unwrap();
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

fn day_2_helper(mut content: Vec<i32>, first: i32, second: i32) -> Option<i32> {
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
    let file = fs::read_to_string("inputs\\day2.txt").unwrap();
    let content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    day_2_helper(content, 12, 2).unwrap()
}

fn day_2_2() -> i32 {
    let file = fs::read_to_string("inputs\\day2.txt").unwrap();
    let content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..99 {
        for j in 0..99 {
            if day_2_helper(content.clone(), i, j) == Some(19690720) {
                return 100 * i + j;
            }
        }
    }
    0
}

fn main() {
    assert_eq!(day_1_1(), 3305301);
    assert_eq!(day_1_2(), 4955106);
    assert_eq!(day_2_1(), 7210630);
    assert_eq!(day_2_2(), 3892);
}
