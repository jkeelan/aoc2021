//use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
// mod days;

fn main() {
    let file_in = fs::File::open("data/day5.txt").unwrap();
    let file_reader = BufReader::new(file_in);
    let res: Vec<Vec<i32>> = file_reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" -> ")
                .map(|v| {
                    v.split(",")
                        .flat_map(|v| v.parse::<i32>())
                        .collect::<Vec<i32>>()
                })
                .flatten()
                .collect()
        })
        .collect();
    let mut counts = HashMap::new();
    let _test = res
        .iter()
        .map(|v| {
            let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);
            if x1 == x2 {
                let sy = if y1 >= y2 { y2 } else { y1 };
                let ey = if y1 >= y2 { y1 } else { y2 };
                for y in sy..=ey {
                    let count = counts.get(&(x1, y)).cloned().unwrap_or(0);
                    counts.insert((x1, y), count + 1);
                }
            } else if y1 == y2 {
                let sx = if x1 >= x2 { x2 } else { x1 };
                let ex = if x1 >= x2 { x1 } else { x2 };
                for x in sx..=ex {
                    let count = counts.get(&(x, y1)).cloned().unwrap_or(0);
                    counts.insert((x, y1), count + 1);
                }
            } else {
                let xdir = if x1 > x2 { -1 } else { 1 };
                let ydir = if y1 > y2 { -1 } else { 1 };
                let (mut x, mut y) = (x1, y1);
                for _ in x1 * xdir..=x2 * xdir {
                    let count = counts.get(&(x, y)).cloned().unwrap_or(0);
                    counts.insert((x, y), count + 1);
                    x += xdir;
                    y += ydir;
                }
            }
        })
        .collect::<()>();
    let mut cross_count = 0;
    for (key, value) in counts {
        if value >= 2 {
            cross_count += 1;
        }
    }
    println!("{:?}", cross_count)
}
