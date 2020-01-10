#![feature(test)]

use std::fs;

extern crate test;

#[cfg(test)]
mod day {
    use super::*;
    use test::Bencher;

    #[bench]
    fn _01_1(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day1.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_01::first(&file), 3_305_301));
    }

    #[bench]
    fn _01_2(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day1.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_01::second(&file), 4_955_106));
    }

    #[bench]
    fn _02_1(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day2.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_02::first(&file), 7_210_630));
    }

    #[bench]
    fn _02_2(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day2.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_02::second(&file), 3_892));
    }

    #[bench]
    fn _03_1(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day3.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_03::first(&file), 1_264));
    }

    #[bench]
    fn _03_2(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day3.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_03::second(&file), 37_390));
    }

    #[bench]
    fn _04_1(b: &mut Bencher) {
        b.iter(|| assert_eq!(advent_of_code_2019::day_04::first(168_630, 718_098), 1_686));
    }

    #[bench]
    fn _04_2(b: &mut Bencher) {
        b.iter(|| assert_eq!(advent_of_code_2019::day_04::second(168_630, 718_098), 1_145));
    }
}

fn main() {
    // for debug
    let file01 = fs::read_to_string("inputs/day1.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_01::first(&file01), 3_305_301);
    assert_eq!(advent_of_code_2019::day_01::second(&file01), 4_955_106);
    let file02 = fs::read_to_string("inputs/day2.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_02::first(&file02), 7_210_630);
    assert_eq!(advent_of_code_2019::day_02::second(&file02), 3_892);
    let file03 = fs::read_to_string("inputs/day3.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_03::first(&file03), 1_264);
    assert_eq!(advent_of_code_2019::day_03::second(&file03), 37_390);
    assert_eq!(advent_of_code_2019::day_04::first(168_630, 718_098), 1_686);
    assert_eq!(advent_of_code_2019::day_04::second(168_630, 718_098), 1_145);
}
