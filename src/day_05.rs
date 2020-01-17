struct Instruction {
    b: u8,
    c: u8,
    de: u8,
}

impl Instruction {
    fn new(mut number: i32) -> Instruction {
        let b = (number / 1_000) as u8;
        number %= 1_000;
        let c = (number / 100) as u8;
        let de = (number % 100) as u8;
        Instruction { b, c, de }
    }
}

fn get_two_parameters(content: &Vec<i32>, ins: &Instruction, index: usize) -> (i32, i32) {
    let first = match ins.c {
        0 => content[content[index + 1] as usize],
        1 => content[index + 1],
        _ => panic!("unknown c"),
    };
    let second = match ins.b {
        0 => content[content[index + 2] as usize],
        1 => content[index + 2],
        _ => panic!("unknown b"),
    };
    (first, second)
}

pub fn first(file: &str, input: i32) -> i32 {
    let mut result = 0;
    let mut content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut index = 0;
    while content[index] != 99 {
        let ins = Instruction::new(content[index]);
        match ins.de {
            1 | 2 => {
                let res_index = content[index + 3] as usize;
                let (first, second) = get_two_parameters(&content, &ins, index);
                content[res_index] = match ins.de {
                    1 => first + second,
                    2 => first * second,
                    _ => panic!("Unknown opcode"),
                };
                index += 4;
            }
            3 => {
                let res_index = content[index + 1] as usize;
                content[res_index] = input;
                index += 2;
            }
            4 => {
                result = match ins.c {
                    0 => content[content[index + 1] as usize],
                    1 => content[index + 1],
                    _ => panic!("Unknown c"),
                };
                index += 2;
            }
            _ => panic!("Unknown opcode"),
        };
    }
    result
}

pub fn second(file: &str, input: i32) -> i32 {
    let mut result = 0;
    let mut content: Vec<i32> = file.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut index = 0;
    while content[index] != 99 {
        let ins = Instruction::new(content[index]);
        match ins.de {
            1 | 2 => {
                let res_index = content[index + 3] as usize;
                let (first, second) = get_two_parameters(&content, &ins, index);
                content[res_index] = match ins.de {
                    1 => first + second,
                    2 => first * second,
                    _ => panic!("Unknown opcode"),
                };
                index += 4;
            }
            3 => {
                let res_index = content[index + 1] as usize;
                content[res_index] = input;
                index += 2;
            }
            4 => {
                result = match ins.c {
                    0 => content[content[index + 1] as usize],
                    1 => content[index + 1],
                    _ => panic!("Unknown c"),
                };
                index += 2;
            }
            5 => {
                let (first, second) = get_two_parameters(&content, &ins, index);
                if first != 0 {
                    index = second as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                let (first, second) = get_two_parameters(&content, &ins, index);
                if first == 0 {
                    index = second as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let (first, second) = get_two_parameters(&content, &ins, index);
                let third = content[index + 3];
                if first < second {
                    content[third as usize] = 1;
                } else {
                    content[third as usize] = 0;
                }
                index += 4;
            }
            8 => {
                let (first, second) = get_two_parameters(&content, &ins, index);
                let third = content[index + 3];
                if first == second {
                    content[third as usize] = 1;
                } else {
                    content[third as usize] = 0;
                }
                index += 4;
            }
            _ => panic!("Unknown opcode"),
        };
    }
    result
}
