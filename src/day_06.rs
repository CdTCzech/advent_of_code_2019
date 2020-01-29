use std::collections::{HashMap, HashSet};

fn sum_orbits(map: &HashMap<&str, HashSet<&str>>, current: &str, depth: i32, sum: &mut i32) {
    let neighbors = match map.get(current) {
        Some(v) => v,
        None => return,
    };
    for neighbor in neighbors {
        *sum += depth;
        sum_orbits(map, neighbor, depth + 1, sum);
    }
}

fn distance(
    map: &HashMap<&str, HashSet<&str>>,
    current: &str,
    wanted: &str,
    depth: i32,
    visited: &mut HashSet<String>,
) -> i32 {
    if current == wanted {
        return depth;
    }
    let neighbors = match map.get(current) {
        Some(v) => v,
        None => return 0,
    };
    visited.insert(current.to_string());
    for neighbor in neighbors {
        if visited.contains(&neighbor.to_string()) {
            continue;
        }
        let res = distance(map, neighbor, wanted, depth + 1, visited);
        if res > 0 {
            return res;
        }
    }
    0
}

pub fn first(file: &str) -> i32 {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut sum = 0;
    for line in file.lines() {
        let connection: Vec<&str> = line.split(')').collect();
        let value = map.entry(connection[0]).or_insert(HashSet::new());
        value.insert(connection[1]);
    }
    sum_orbits(&map, "COM", 1, &mut sum);
    sum
}

pub fn second(file: &str) -> i32 {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in file.lines() {
        let connection: Vec<&str> = line.split(')').collect();
        let value1 = map.entry(connection[0]).or_insert(HashSet::new());
        value1.insert(connection[1]);
        let value2 = map.entry(connection[1]).or_insert(HashSet::new());
        value2.insert(connection[0]);
    }
    distance(&map, "YOU", "SAN", 0, &mut HashSet::new())
}
