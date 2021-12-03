use std::{fs, io};

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
