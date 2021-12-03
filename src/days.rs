use std::fs;

pub fn day_1(path: &str) {
    let mut count = 0;
    let accum = |acc, v: f32| {
        if acc > -1.0 && v > acc {
            count += 1
        }
        return v;
    };
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .flat_map(|s| s.parse::<f32>())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|v| v.iter().sum())
        .fold(-1.0, accum);

    println!("day 1: {:?}", count)
}

pub fn day_2(path: &str) {
    let val = fs::read_to_string(path).unwrap();
    let mut f = 0;
    let mut depth = 0;
    let mut aim = 0;
    let _result: Vec<_> = val
        .split("\n")
        .into_iter()
        .flat_map(|s| -> Result<(&str, i32), &'static str> {
            let split = s.split(" ").collect::<Vec<&str>>();
            return match split.as_slice() {
                [dir, value] => match (dir, value.parse::<i32>()) {
                    (&"forward", Ok(value)) => {
                        f += value;
                        depth += value * aim;
                        Ok(("forward", value))
                    }
                    (&"up", Ok(value)) => {
                        aim -= value;
                        Ok(("up", value))
                    }
                    (&"down", Ok(value)) => {
                        aim += value;
                        Ok(("down", value))
                    }
                    _ => Err("Unable to parse int"),
                },

                [..] => Err("Bad line"),
            };
        })
        .collect();
    println!("day 2: {:?}", f * depth)
}

fn get_mcv(remaining: &Vec<Vec<u32>>, inv: bool) -> Vec<u32> {
    let mcv: Vec<_> = remaining
        .iter()
        .fold(Vec::new(), |accum, v| {
            if accum.len() == 0 {
                return v.to_vec();
            } else if v.len() == 0 {
                return accum.to_vec();
            }
            v.iter().zip(accum).map(|(v, x)| v + x).collect::<Vec<_>>()
        })
        .iter()
        .map(|ones| {
            let zeros = &(remaining.len() as u32 - ones);
            if !inv {
                (ones >= zeros) as u32
            } else {
                (zeros > ones) as u32
            }
        })
        .collect();
    return mcv;
}

fn get_rating(arr: &Vec<Vec<u32>>, inv: bool) -> Vec<u32> {
    let mut index = 0;
    let mut remaining = arr.to_vec();
    while remaining.len() > 1 {
        let mcv = get_mcv(&remaining, inv);
        remaining = remaining
            .into_iter()
            .filter(|v| v[index] == mcv[index])
            .collect::<Vec<_>>();
        index += 1;
    }
    return remaining[0].to_vec();
}

pub fn day_3(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let v: Vec<_> = contents.split("\n").collect();
    let arr: Vec<Vec<_>> = v
        .iter()
        .map(|s| s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect())
        .filter(|v: &Vec<_>| v.len() > 0)
        .collect();

    let ox = get_rating(&arr, false)
        .iter()
        .fold(0, |acc, b| (acc << 1) + b);
    let co2 = get_rating(&arr, true)
        .iter()
        .fold(0, |acc, b| (acc << 1) + b);
    println!("day 3: {}", ox * co2)
}
