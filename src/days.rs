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

pub fn day_4() {
    let ticket_dim = 5;
    let mut row_counts = HashMap::new();
    let mut col_counts = HashMap::new();
    let mut number_to_ticket = HashMap::new();
    let mut sums = Vec::new();
    let mut winners = Vec::new();
    let mut winner_record = HashSet::new();
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
    for (key, value) in number_to_ticket {
        if value.len() == 0 {
            println!("{:?}", key);
        }
    }
    println!("{:?}", winners[0]);
    println!("{:?}", winners.last().unwrap());
}

pub fn day_5() {
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

fn day_6() {
    let mut nums: Vec<i64> = fs::read_to_string("data/day6.txt")
        .unwrap()
        .split(",")
        .into_iter()
        .flat_map(|v| v.trim().parse::<i64>())
        .collect();
    let mut counts = HashMap::new();
    for num in nums {
        let &val = counts.get(&num).unwrap_or(&0);
        counts.insert(num, val + 1 as i64);
    }
    for _ in 0..256 {
        let mut new_counts = HashMap::new();
        for value in 0..=8 {
            let &old = counts.get(&value).unwrap_or(&0);
            let new_val = if value > 0 { value - 1 } else { 6 };
            if value == 0 {
                new_counts.insert(8, old);
            }
            let &new_count = new_counts.get(&new_val).unwrap_or(&0);
            new_counts.insert(new_val, new_count + old);
        }
        counts = new_counts;
    }

    let sum = counts.iter().fold(0, |accum, (_, &v)| accum + v);
    println!("{}", sum);
}

// I'm naughty and did day 7 in an ipython terminal. Part 1 was the median, part 2 was the mean.
