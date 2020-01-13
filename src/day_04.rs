use std::cmp::Ordering;

fn i32_to_vec_u8(number: i32) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect()
}

fn make_incremental(start: &mut i32, number: &mut Vec<u8>) {
    let mut mul = 10_000;
    for i in 0..5 {
        if number[i] > number[i + 1] {
            *start += (number[i] - number[i + 1]) as i32 * mul;
            number[i + 1] = number[i];
        }
        mul /= 10;
    }
}

fn add_one(start: &mut i32, number: &mut Vec<u8>, index: usize) {
    number[index] += 1;
    if number[index] == 10 {
        number[index] = 0;
        add_one(start, number, index - 1);
        if index == 5 {
            make_incremental(start, number)
        }
    }
    if index == 5 {
        *start += 1;
    }
}

fn first_check_doubles(number: &mut Vec<u8>) -> i32 {
    let mut double = 0;
    for i in (0..5).rev() {
        if number[i] == number[i + 1] {
            double += 1;
        }
    }
    double
}

fn second_check_doubles(number: &mut Vec<u8>) -> i32 {
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

fn implementation(mut start: i32, end: i32, check_doubles: fn(&mut Vec<u8>) -> i32) -> i32 {
    let mut number = i32_to_vec_u8(start);
    let mut result = 0;
    make_incremental(&mut start, &mut number);
    while start < end {
        let double = check_doubles(&mut number);
        if double > 0 {
            result += 1;
        }
        add_one(&mut start, &mut number, 5);
    }
    result
}

pub fn first(start: i32, end: i32) -> i32 {
    implementation(start, end, first_check_doubles)
}

pub fn second(start: i32, end: i32) -> i32 {
    implementation(start, end, second_check_doubles)
}
