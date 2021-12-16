use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

fn main() -> Result<()> {
    let mut f = BufReader::new(File::open("input")?);
    let mut buf = String::new();
    let mut lines = Vec::new();
    while f.read_line(&mut buf)? > 0 {
        lines.push(buf.trim().to_string());
        buf.clear();
    }
    let list = part_one(&lines)?;
    for i in list.iter().rev() {
        lines.remove(*i);
    }
    part_two(&lines);
    Ok(())
}

fn part_two(lines: &[String]) {
    let sc_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores = Vec::new();
    for line in lines {
        let mut score = 0i64;
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => unreachable!(),
            }
        }
        while !stack.is_empty() {
            score *= 5;
            score += sc_map[&stack.pop().unwrap()];
        }
        scores.push(score);
    }

    scores.sort();
    dbg!(scores[scores.len() / 2]);
}

fn part_one(lines: &[String]) -> Result<Vec<usize>> {
    let mut ans = 0;
    let mut remove_items = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if *stack.last().unwrap() == '(' {
                        stack.pop();
                    } else {
                        ans += 3;
                        remove_items.push(i);
                        break;
                    }
                }
                ']' => {
                    if *stack.last().unwrap() == '[' {
                        stack.pop();
                    } else {
                        ans += 57;
                        remove_items.push(i);
                        break;
                    }
                }
                '}' => {
                    if *stack.last().unwrap() == '{' {
                        stack.pop();
                    } else {
                        ans += 1197;
                        remove_items.push(i);
                        break;
                    }
                }
                '>' => {
                    if *stack.last().unwrap() == '<' {
                        stack.pop();
                    } else {
                        ans += 25137;
                        remove_items.push(i);
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    dbg!(ans);
    Ok(remove_items)
}
