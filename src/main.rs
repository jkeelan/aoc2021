//use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn is_close_bracket(c: &char) -> bool {
    match c {
        '>' => return true,
        '}' => return true,
        ']' => return true,
        ')' => return true,
        _ => return false,
    }
}

fn is_open_bracket(c: &char) -> bool {
    match c {
        '<' => return true,
        '{' => return true,
        '[' => return true,
        '(' => return true,
        _ => return false,
    }
}
// mod days;
fn main() {
    let scores = HashMap::from([('}', 3), (')', 1), (']', 2), ('>', 4)]);
    let matcher = HashMap::from([('{', '}'), ('[', ']'), ('<', '>'), ('(', ')')]);
    let mut score = 0;
    let mut line_scores: Vec<i64> = Vec::new();
    let file_contents = std::fs::read_to_string("data/day10.txt").unwrap();
    let lines = file_contents.split("\n").collect::<Vec<&str>>();
    for line in lines {
        let mut buffer: Vec<char> = Vec::new();
        let mut good_line = true;
        for c in line.chars() {
            if is_open_bracket(&c) {
                buffer.push(c);
            } else if buffer.len() > 0 && is_close_bracket(&c) {
                let old_bracket = buffer.pop().unwrap();
                match (old_bracket, c) {
                    ('{', '}') => {}
                    ('[', ']') => {}
                    ('<', '>') => {}
                    ('(', ')') => {}
                    (_, new) => {
                        good_line = false;
                        break;
                    }
                }
            } else {
                println!("{}", c);
                good_line = false;
                break;
            }
        }
        let mut line_sum: i64 = 0;
        if good_line {
            let mut auto_complete = buffer
                .iter()
                .map(|v| scores.get(matcher.get(&v).unwrap()).unwrap())
                .collect::<Vec<&i32>>();
            auto_complete.reverse();
            for value in auto_complete {
                line_sum *= 5;
                line_sum += *value as i64;
            }
            line_scores.push(line_sum);
        }
    }
    line_scores.sort();

    println!("Sum is: {}", line_scores[line_scores.len() / 2 as usize]);
}
