//use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
// mod days;
fn read_input(filename: &str) -> io::Result<(Vec<i32>, Vec<Vec<i32>>)> {
    let file_in = fs::File::open(filename)?;
    let mut file_reader = BufReader::new(file_in);
    let mut line_buf = String::new();
    let _res = file_reader.read_line(&mut line_buf);
    let nums: Vec<i32> = line_buf
        .split(",")
        .into_iter()
        .flat_map(|v| v.parse::<i32>())
        .collect();
    let arr: Vec<Vec<i32>> = file_reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .flat_map(|number| number.parse::<i32>())
                .collect()
        })
        .collect();
    return Ok((nums, arr.into_iter().filter(|v| v.len() > 0).collect()));
}
fn main() {
    let ticket_dim = 5;
    let mut row_counts = HashMap::new();
    let mut col_counts = HashMap::new();
    let mut number_to_ticket = HashMap::new();
    let mut sums = Vec::new();
    let (nums, arr) = read_input("data/day4.txt").unwrap();
    for (i, chunk) in arr.chunks(ticket_dim).enumerate() {
        let mut sum = 0;
        for (r, row) in chunk.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                let mut v = number_to_ticket.get(val).cloned().unwrap_or(Vec::new());
                v.push((i, r, c));
                number_to_ticket.insert(val, v);
                row_counts.insert((i, r), 0);
                col_counts.insert((i, c), 0);
                sum += val;
            }
        }
        sums.push(sum);
    }
    let mut winners = Vec::new();
    let mut winner_record = HashSet::new();
    for num in nums {
        let locs = number_to_ticket.get(&num).cloned().unwrap_or(Vec::new());
        for loc in locs {
            sums[loc.0] -= num;
            let row_count = row_counts.get(&(loc.0, loc.1)).unwrap();
            let col_count = col_counts.get(&(loc.0, loc.2)).unwrap();
            let new_r_count = row_count + 1;
            let new_c_count = col_count + 1;
            row_counts.insert((loc.0, loc.1), new_r_count);
            col_counts.insert((loc.0, loc.2), new_c_count);
            if new_r_count == ticket_dim || new_c_count == ticket_dim {
                let total = sums[loc.0] * num;
                if !winner_record.contains(&loc.0) {
                    winners.push((loc.0, total));
                    winner_record.insert(loc.0);
                }
            }
        }
    }
    println!("{:?}", winners[0]);
    println!("{:?}", winners.last().unwrap());
}
