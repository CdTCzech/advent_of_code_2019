#![feature(test)]

use std::fs;

extern crate test;

#[cfg(test)]
mod day {
    use super::*;
    use test::Bencher;

    #[bench]
    fn _01_1(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day_01.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_01::first(&file), 3_305_301));
    }

    #[bench]
    fn _01_2(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day_01.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_01::second(&file), 4_955_106));
    }

    #[bench]
    fn _02_1(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day_02.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_02::first(&file), 7_210_630));
    }

    #[bench]
    fn _02_2(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day_02.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_02::second(&file), 3_892));
    }

    #[bench]
    fn _03_1(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day_03.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_03::first(&file), 1_264));
    }

    #[bench]
    fn _03_2(b: &mut Bencher) {
        let file = fs::read_to_string("inputs/day_03.txt").unwrap();
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

    #[bench]
    fn _05_1(b: &mut Bencher) {
        let file_05 = fs::read_to_string("inputs/day_05.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_05::first(&file_05, 1), 6_761_139));
    }

    #[bench]
    fn _05_2(b: &mut Bencher) {
        let file_05 = fs::read_to_string("inputs/day_05.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_05::second(&file_05, 5), 9_217_546));
    }

    #[bench]
    fn _06_1(b: &mut Bencher) {
        let file_06 = fs::read_to_string("inputs/day_06.txt").unwrap();
        b.iter(|| assert_eq!(advent_of_code_2019::day_06::first(&file_06), 0));
    }
}

fn main() {
    // for debug
    let file_01 = fs::read_to_string("inputs/day_01.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_01::first(&file_01), 3_305_301);
    assert_eq!(advent_of_code_2019::day_01::second(&file_01), 4_955_106);
    let file_02 = fs::read_to_string("inputs/day_02.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_02::first(&file_02), 7_210_630);
    assert_eq!(advent_of_code_2019::day_02::second(&file_02), 3_892);
    let file_03 = fs::read_to_string("inputs/day_03.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_03::first(&file_03), 1_264);
    assert_eq!(advent_of_code_2019::day_03::second(&file_03), 37_390);
    assert_eq!(advent_of_code_2019::day_04::first(168_630, 718_098), 1_686);
    assert_eq!(advent_of_code_2019::day_04::second(168_630, 718_098), 1_145);
    let file_05 = fs::read_to_string("inputs/day_05.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_05::first(&file_05, 1), 6_761_139);
    assert_eq!(advent_of_code_2019::day_05::second(&file_05, 5), 9_217_546);
    let file_06 = fs::read_to_string("inputs/day_06.txt").unwrap();
    assert_eq!(advent_of_code_2019::day_06::first(&file_06), 0);
}
