use crate::day_05;

pub fn first(file: &str) -> i32 {
    let mut res = 0;
    for i1 in 0..5 {
        let res1 = day_05::second(file, &vec![i1, 0]);
        for i2 in 0..5 {
            if i2 == i1 {
                continue;
            }
            let res2 = day_05::second(file, &vec![i2, res1]);
            for i3 in 0..5 {
                if i3 == i1 || i3 == i2 {
                    continue;
                }
                let res3 = day_05::second(file, &vec![i3, res2]);
                for i4 in 0..5 {
                    if i4 == i1 || i4 == i2 || i4 == i3 {
                        continue;
                    }
                    let res4 = day_05::second(file, &vec![i4, res3]);
                    for i5 in 0..5 {
                        if i5 == i1 || i5 == i2 || i5 == i3 || i5 == i4 {
                            continue;
                        }
                        let res5 = day_05::second(file, &vec![i5, res4]);
                        if res5 > res {
                            res = res5;
                        }
                    }
                }
            }
        }
    }
    res
}
