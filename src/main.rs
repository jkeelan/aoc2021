use std::fs;
mod days;

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

fn main() {
    let contents = fs::read_to_string("data/day3_test.txt").unwrap();
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
    println!("{}", ox * co2)
}
