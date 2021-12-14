use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    // part_one()?;
    part_two()?;
    Ok(())
}

fn part_two() -> Result<()> {
    // a:8, b:6, c:8, d:7, e:4, f:9, g:7
    let mut f = BufReader::new(File::open("input.txt")?);
    let mut buf = String::new();
    let mut tens = Vec::new();
    let mut outputs = Vec::new();
    while f.read_line(&mut buf)? > 0 {
        let (p1, p2) = buf.split_once("|").context("| not found")?;
        let ten: Vec<&str> = p1.trim().split_whitespace().collect();
        tens.push(
            ten.iter()
                .map(|s| {
                    let mut v = s.as_bytes().to_vec();
                    v.sort();
                    v
                })
                .collect::<Vec<_>>(),
        );
        let output: Vec<&str> = p2.trim().split_whitespace().collect();
        outputs.push(
            output
                .iter()
                .map(|s| {
                    let mut v = s.as_bytes().to_vec();
                    v.sort();
                    v
                })
                .collect::<Vec<_>>(),
        );
        buf.clear();
    }
    for i in 0..tens.len() {
        let mut occurs: HashMap<u8, i32> = HashMap::new();
        let mut byte_map: HashMap<u8, u8> = HashMap::new();
        let mut one = &tens[0][0];
        let mut four = &tens[0][0];
        let mut seven = &tens[0][0];
        let mut eight = &tens[0][0];
        for word in &tens[i] {
            match word.len() {
                2 => one = word,
                4 => four = word,
                3 => seven = word,
                7 => eight = word,
                _ => (),
            };
            for c in word {
                *occurs.entry(*c).or_default() += 1;
            }
        }
        dbg!(&tens[i]);
        dbg!(&occurs);
        for (k, v) in occurs {
            match v {
                4 => {
                    // e
                    byte_map.insert(k, 'e' as u8);
                }
                6 => {
                    // b
                    byte_map.insert(k, 'b' as u8);
                }
                9 => {
                    // f
                    byte_map.insert(k, 'f' as u8);
                    // c
                    if one[0] == k {
                        byte_map.insert(one[1], 'c' as u8);
                    } else {
                        byte_map.insert(one[0], 'c' as u8);
                    }
                }
                _ => (),
            };
        }
        break;
    }
    Ok(())
}

fn part_one() -> Result<()> {
    let mut f = BufReader::new(File::open("input.txt")?);
    let mut buf = String::new();
    let mut ans = 0;
    while f.read_line(&mut buf)? > 0 {
        let (_, p2) = buf.split_once("|").unwrap();
        let words: Vec<&str> = p2.trim().split_whitespace().collect();
        ans += words.iter().fold(0, |acc, s| {
            acc + match s.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        });
        buf.clear();
    }
    dbg!(ans);
    Ok(())
}
