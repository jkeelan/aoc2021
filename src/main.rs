//use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
// mod days;

fn share_char(a: &str, b: &str) -> String {
    // get which one is shorter
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };

    // fill the set with the characters from the shorter string
    let set: HashSet<char> = shorter.chars().collect();

    let v = longer
        .chars()
        .filter(|c| set.contains(&c))
        .collect::<String>();
    return v;
}

fn matched(a: &str, b: &str) -> bool {
    let overlap = share_char(a, b);
    if overlap.len() == a.len() && a.len() == b.len() {
        return true;
    }
    return false;
}

fn main() {
    let mut segments = HashMap::new();
    segments.insert(0, "abcefg");
    segments.insert(1, "cf");
    segments.insert(2, "acdeg");
    segments.insert(3, "acdfg");
    segments.insert(4, "bcdf");
    segments.insert(5, "abdfg");
    segments.insert(6, "abdefg");
    segments.insert(7, "acf");
    segments.insert(8, "abcdefg");
    segments.insert(9, "abcdfg");

    let file_contents = fs::read_to_string("data/day8.txt").unwrap();
    let contents: Vec<(Vec<&str>, Vec<&str>)> = file_contents
        .split("\n")
        .into_iter()
        .map(|x| {
            let mut split = x.split(" | ");
            let lhs = split.next().unwrap().split(" ").collect::<Vec<&str>>();
            let rhs = split.next().unwrap().split(" ").collect::<Vec<&str>>();
            (lhs, rhs)
        })
        .collect();
    // Calculate baseline signatures
    let mut signatures: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in [0, 2, 3, 5, 6, 9] {
        let mut temp = HashSet::new();
        for j in [1, 4, 7, 8] {
            temp.insert(
                share_char(segments.get(&i).unwrap(), segments.get(&j).unwrap()).len() as i32,
            );
        }
        signatures.insert(i, temp);
    }
    let mut total_sum = 0;
    for (line, res) in contents {
        let lens: Vec<i32> = line.iter().map(|v| v.len() as i32).collect();
        let mut line_map = HashMap::new();
        for i in 0..lens.len() {
            match lens[i] {
                2 => line_map.insert(1, line[i]),
                3 => line_map.insert(7, line[i]),
                4 => line_map.insert(4, line[i]),
                7 => line_map.insert(8, line[i]),
                _ => Some("lol"),
            };
        }
        // calculate signatures for the remaining numbers
        for i in 0..line.len() {
            let mut temp = HashSet::new();
            // iterate over 1, 4, 7, 8
            for k in [1, 4, 7, 8] {
                // Calculate signature as the number of overlapping characters
                let other = line_map.get(&k).unwrap();
                temp.insert(share_char(other, line[i]).len() as i32);
            }
            for (j, main_sig) in &signatures {
                if &temp == main_sig {
                    line_map.insert(*j, line[i]);
                }
            }
        }

        // convert res and sum
        let mut sum = 0;
        let mut mult = 1000;
        for v in res {
            for (num, s) in &line_map {
                if matched(v, s) {
                    sum += mult * num;
                    mult = mult / 10;
                }
            }
        }
        total_sum += sum;
    }
    println!("{}", total_sum);
}
