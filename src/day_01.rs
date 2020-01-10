pub fn first(file: &str) -> i32 {
    let mut sum = 0;
    for line in file.lines() {
        let num = line.parse::<i32>().unwrap();
        sum += num / 3 - 2;
    }
    sum
}

pub fn second(file: &str) -> i32 {
    let mut sum = 0;
    for line in file.lines() {
        let mut num = line.parse::<i32>().unwrap();
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
