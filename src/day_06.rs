use std::collections::{HashMap, HashSet};

fn traverse(map: &HashMap<&str, HashSet<&str>>, current: &str, depth: i32, sum: &mut i32) {
    let neighbors = match map.get(current) {
        Some(v) => v,
        None => return,
    };
    for neighbor in neighbors {
        *sum += depth;
        traverse(map, neighbor, depth + 1, sum);
    }
}

pub fn first(file: &str) -> i32 {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut sum = 0;
    for line in file.lines() {
        let connection: Vec<&str> = line.split(')').collect();
        let value = map.entry(connection[0]).or_insert(HashSet::new());
        value.insert(connection[1]);
    }
    traverse(&map, "COM", 1, &mut sum);
    sum
}
