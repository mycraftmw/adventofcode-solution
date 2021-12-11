use std::{fs::File, io::Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut report = read_from_file("input")?;
    part_one(&report)?;
    part_two(&mut report)?;
    Ok(())
}

fn part_two(report: &mut Vec<String>) -> Result<()> {
    report.sort();
    let (mut l, mut r) = (0 as usize, report.len());
    let mut idx = 0;
    let max = report[0].len();
    // find the oxygen
    while l < r - 1 && idx < max {
        let f1 = l + cut(&report[l..r], idx)?;
        if f1 - l > r - f1 {
            r = f1;
        } else {
            l = f1;
        }
        idx += 1;
    }
    let oxygen = i32::from_str_radix(&report[l], 2)?;
    l = 0;
    r = report.len();
    idx = 0;
    // find the co2
    while l < r - 1 && idx < max {
        let f1 = l + cut(&report[l..r], idx)?;
        if f1 - l > r - f1 {
            l = f1;
        } else {
            r = f1;
        }
        idx += 1;
    }
    let co2 = i32::from_str_radix(&report[l], 2)?;
    println!("{}", oxygen * co2);
    Ok(())
}

fn cut(source: &[String], idx: usize) -> Result<usize> {
    let (mut l, mut r) = (0, source.len());
    while l < r {
        let mid = l + ((r - l) >> 1);
        if source[mid].chars().nth(idx).unwrap() == '1' {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    Ok(l)
}

fn part_one(report: &[String]) -> Result<()> {
    let mut sum = report[0].chars().map(|_| 0).collect::<Vec<_>>();
    for e in report {
        for (i, c) in e.char_indices() {
            sum[i] += match c {
                '1' => 1,
                _ => 0,
            }
        }
    }
    let threshold = report.len() / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in sum {
        gamma <<= 1;
        epsilon <<= 1;
        if i > threshold as i32 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    println!("{}", gamma * epsilon);
    Ok(())
}

fn read_from_file(path: &str) -> Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf
        .trim()
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.to_string())
        .collect())
}
