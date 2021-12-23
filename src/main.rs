//use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn flash(grid: &mut Vec<Vec<i32>>, flashers: &mut Vec<(usize, usize)>, i: i32, j: i32) -> i32 {
    let mut count = 0;
    if grid[i as usize][j as usize] <= 9 || flashers.contains(&(i as usize, j as usize)) {
        return count;
    }
    count += 1;
    flashers.push((i as usize, j as usize));
    for r in -1..=1 {
        for c in -1..=1 {
            if r == 0 && c == 0 {
                continue;
            }
            if i + r < 0 || i + r >= grid.len() as i32 {
                continue;
            }
            if j + c < 0 || j + c >= grid[0].len() as i32 {
                continue;
            }
            let x = (i + r) as usize;
            let y = (j + c) as usize;
            grid[x][y] += 1;
            if !flashers.contains(&(x, y)) && grid[x][y] > 9 {
                count += flash(grid, flashers, x as i32, y as i32);
            }
        }
    }
    return count;
}
// mod days;
fn main() {
    let lines = std::fs::read_to_string("data/day11.txt").unwrap();
    let mut grid: Vec<Vec<i32>> = lines
        .split("\n")
        .map(|v| {
            v.chars()
                .flat_map(|n| n.to_string().parse::<i32>())
                .collect()
        })
        .collect();
    let mut total_flashes = 0;
    for step in 0..1000 {
        let mut flashers: Vec<(usize, usize)> = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                grid[i][j] += 1;
                if grid[i][j] > 9 {
                    flashers.push((i, j));
                }
            }
        }
        let mut flash_record: Vec<(usize, usize)> = Vec::new();
        let mut flash_count = 0;
        for (i, j) in flashers {
            flash_count += flash(&mut grid, &mut flash_record, i as i32, j as i32);
        }
        if flash_count == (grid.len() * grid[0].len()) as i32 {
            println!("ALL FLASH @ {}", step + 1)
        }
        total_flashes += flash_count;
        for (i, j) in flash_record {
            grid[i][j] = 0;
        }
    }
    println!("total flashes: {}", total_flashes)
}
