use std::fs;
mod days;

fn main() {
    days::day_1("data/day1.txt");
    days::day_2("data/day2.txt");
    let folder = |accum: Vec<i32>, v: Vec<_>| {
        if accum.len() == 0 {
            return v;
        } else if v.len() == 0 {
            return accum;
        }
        v.iter().zip(accum).map(|(v, x)| v + x).collect::<Vec<_>>()
    };
    let contents = fs::read_to_string("data/day3.txt").unwrap();
    let v: Vec<_> = contents.split("\n").collect();
    let count = v.len();
    let folded = v
        .iter()
        .map(|s| s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect())
        .fold(Vec::new(), folder);
    let vals = folded
        .iter()
        .map(|v| (v > &(count as i32 / 2)))
        .fold((0, 0), |acc, b| {
            (acc.0 * 2 + b as u32, acc.1 * 2 + !b as u32)
        });
    println!("{:?}", vals.1 * vals.0);
}
