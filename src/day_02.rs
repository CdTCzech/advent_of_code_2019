fn implementation(mut content: Vec<i32>, first: i32, second: i32) -> Option<i32> {
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

pub fn first(file: &str) -> i32 {
    let content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    implementation(content, 12, 2).unwrap()
}

pub fn second(file: &str) -> i32 {
    let content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..99 {
        for j in 0..99 {
            if implementation(content.clone(), i, j) == Some(19_690_720) {
                return 100 * i + j;
            }
        }
    }
    0
}
