use std::{
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
    part_one(&mut lines);
    Ok(())
}

fn part_one(lines: &[String]) {
    let mut ans = 0;
    for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if *stack.last().unwrap() == '(' {
                        stack.pop();
                    } else {
                        ans += 3;
                        break;
                    }
                }
                ']' => {
                    if *stack.last().unwrap() == '[' {
                        stack.pop();
                    } else {
                        ans += 57;
                        break;
                    }
                }
                '}' => {
                    if *stack.last().unwrap() == '{' {
                        stack.pop();
                    } else {
                        ans += 1197;
                        break;
                    }
                }
                '>' => {
                    if *stack.last().unwrap() == '<' {
                        stack.pop();
                    } else {
                        ans += 25137;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    dbg!(ans);
}
