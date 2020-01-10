use std::cmp::Ordering;

fn i32_to_vec_u32(number: i32) -> Vec<u32> {
    number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

fn add_one(number: &mut Vec<u32>, index: usize) {
    number[index] += 1;
    if number[index] == 10 {
        number[index] = 0;
        add_one(number, index - 1);
    }
}

fn first_check_doubles(number: &mut Vec<u32>) -> i32 {
    let mut double = 0;
    for i in (0..5).rev() {
        match number[i].cmp(&number[i + 1]) {
            Ordering::Greater => {
                double = 0;
                break;
            }
            Ordering::Equal => double += 1,
            Ordering::Less => (),
        }
    }
    double
}

fn second_check_doubles(number: &mut Vec<u32>) -> i32 {
    let mut double = 0;
    let mut repeats = 0;
    for i in (0..5).rev() {
        match number[i].cmp(&number[i + 1]) {
            Ordering::Greater => {
                double = 0;
                repeats = 0;
                break;
            }
            Ordering::Equal => repeats += 1,
            Ordering::Less => {
                if repeats == 1 {
                    double += 1;
                }
                repeats = 0;
            }
        }
    }
    if repeats == 1 {
        double += 1;
    }
    double
}

fn implementation(mut start: i32, end: i32, check_doubles: fn(&mut Vec<u32>) -> i32) -> i32 {
    let mut number: Vec<u32> = i32_to_vec_u32(start);
    let number_last_index = number.len() - 1;
    let mut result = 0;
    while start != end {
        let double = check_doubles(&mut number);
        if double > 0 {
            result += 1;
        }
        add_one(&mut number, number_last_index);
        start += 1;
    }
    result
}

pub fn first(start: i32, end: i32) -> i32 {
    implementation(start, end, first_check_doubles)
}

pub fn second(start: i32, end: i32) -> i32 {
    implementation(start, end, second_check_doubles)
}
