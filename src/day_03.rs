use std::collections::HashMap;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn increment_direction(direction: &str, current: &mut Point) {
    match direction.get(0..1).unwrap() {
        "D" => current.y -= 1,
        "L" => current.x -= 1,
        "R" => current.x += 1,
        "U" => current.y += 1,
        _ => panic!("Unknown direction"),
    };
}

fn implementation(file: &str) -> (i32, i32) {
    let lines: Vec<&str> = file.split("\r\n").collect();
    let mut wire_1 = HashMap::new();
    let mut current = Point { x: 0, y: 0 };
    let mut step = 0;
    for direction in lines[0].split(',').collect::<Vec<&str>>() {
        let num = direction[1..].parse::<i32>().unwrap();
        for _ in 0..num {
            increment_direction(direction, &mut current);
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
            increment_direction(direction, &mut current);
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

pub fn first(file: &str) -> i32 {
    implementation(&file).0
}

pub fn second(file: &str) -> i32 {
    implementation(&file).1
}
