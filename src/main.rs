//use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
// mod days;

fn get_risk(vals: &Vec<Vec<i32>>) -> i32 {
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut sum_risk = 0;
    for (i, row) in vals.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let mut is_min = true;
            for (dx, dy) in dirs {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x < 0 || y < 0 || x >= vals.len() as i32 || y >= vals[0].len() as i32 {
                    continue;
                } else {
                    if v >= &vals[x as usize][y as usize] {
                        is_min = false
                    }
                }
            }
            if is_min {
                sum_risk += v + 1;
            }
        }
    }
    return sum_risk;
}

fn is_basin(i: usize, j: usize, vals: &Vec<Vec<i32>>) -> bool {
    let v = vals[i][j];
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut is_min = true;
    for (dx, dy) in dirs {
        let x = i as i32 + dx;
        let y = j as i32 + dy;
        if x < 0 || y < 0 || x >= vals.len() as i32 || y >= vals[0].len() as i32 {
            continue;
        } else {
            if v >= vals[x as usize][y as usize] {
                is_min = false
            }
        }
    }
    return is_min;
}
fn find_basins(vals: &Vec<Vec<i32>>) -> &Vec<(usize, usize)> {
    let mut basins = Vec::new();
    for (i, row) in vals.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if is_basin(i, j, vals) {
                basins.push((i, j));
            }
        }
    }
    return &basins;
}

fn find_basin<'a>(
    x: &usize,
    y: &usize,
    vals: &'a Vec<Vec<i32>>,
    basins: &'a mut HashMap<&'a (usize, usize), i32>,
) -> &'a (usize, usize) {
    match is_basin(*x, *y, &vals) {
        true => return &(*x, *y),
        false => {
            let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            let v = vals[*x][*y];
            for (dx, dy) in dirs {
                let k = *x as i32 + dx;
                let l = *y as i32 + dy;
                if k < 0 || l < 0 || k >= vals.len() as i32 || l >= vals[0].len() as i32 {
                    continue;
                } else {
                    if v < vals[k as usize][l as usize] {
                        let basin = find_basin(&(k as usize), &(l as usize), vals, basins);
                        let count = *basins.get(&basin).unwrap_or(&0);
                        basins.insert(basin, count + 1);
                        return basin;
                    }
                }
            }
            return &(0, 0);
        }
    }
}

fn main() {
    let vals = fs::read_to_string("./data/day9.txt")
        .unwrap()
        .split("\n")
        .map(|x| {
            x.split("")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!("Risk is: {}", get_risk(&vals));

    let basins = find_basins(&vals);
    let mut basin_counts: HashMap<&(usize, usize), i32> = HashMap::new();
    let mut basin_cache: HashMap<&(usize, usize), (usize, usize)> = HashMap::new();
    for key in basins {
        basin_counts.insert(key, 0);
    }

    for (i, row) in vals.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v == 9 || basin_counts.contains_key(&(i, j)) {
                continue;
            } else if basin_cache.contains_key(&(i, j)) {
                let basin = basin_cache.get(&(i, j)).unwrap();
                let count = basin_counts.get(basin).unwrap();
                basin_counts.insert(basin, count + 1);
            } else {
                let mut new_basin_counts = HashMap::new();
                let (basin, count) = find_basin(&i, &j, &vals, &mut new_basin_counts);
                for (key, value) in new_basin_counts {
                    let count = *basin_counts.get(key).unwrap();
                }
            }
        }
    }
}
