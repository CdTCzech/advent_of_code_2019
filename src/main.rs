use std::fs::File;
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

fn main() {
    assert_eq!(day_1_1(), 3305301);
    assert_eq!(day_1_2(), 4955106);
}
