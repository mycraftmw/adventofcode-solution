use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_two() -> Result<()> {
    // a:8, b:6, c:8, d:7, e:4, f:9, g:7
    // 0,6,9:6, 2,3,5:5, 1:2, 4:4, 7:3, 8:7
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
                    String::from_utf8(v).unwrap()
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
                    String::from_utf8(v).unwrap()
                })
                .collect::<Vec<_>>(),
        );
        buf.clear();
    }
    let mut ans = 0;
    for (i, list) in tens.iter().enumerate() {
        let mut number2word: HashMap<usize, &String> = HashMap::new();
        let mut word2number: HashMap<String, usize> = HashMap::new();
        let mut five = Vec::new();
        let mut six = Vec::new();
        for word in list {
            match word.len() {
                2 => {
                    number2word.insert(1, word);
                    word2number.insert(word.clone(), 1);
                }
                4 => {
                    number2word.insert(4, word);
                    word2number.insert(word.clone(), 4);
                }
                3 => {
                    number2word.insert(7, word);
                    word2number.insert(word.clone(), 7);
                }
                7 => {
                    number2word.insert(8, word);
                    word2number.insert(word.clone(), 8);
                }
                5 => five.push(word),
                6 => six.push(word),
                _ => unreachable!(),
            }
        }
        // 3
        let mut p = 0;
        for (i, w) in five.iter().enumerate() {
            if dif(number2word[&1], w) == 3 {
                number2word.insert(3, w);
                word2number.insert((*w).clone(), 3);
                p = i;
                break;
            }
        }
        five.remove(p);
        // 9
        for (i, w) in six.iter().enumerate() {
            if dif(number2word[&3], w) == 1 {
                number2word.insert(9, w);
                word2number.insert((*w).clone(), 9);
                p = i;
                break;
            }
        }
        six.remove(p);
        // 0
        for (i, w) in six.iter().enumerate() {
            if dif(number2word[&7], w) == 3 {
                number2word.insert(0, w);
                word2number.insert((*w).clone(), 0);
                p = i;
                break;
            }
        }
        six.remove(p);
        // 6
        number2word.insert(6, six[0]);
        word2number.insert(six[0].clone(), 6);
        // 5 && 2
        if dif(number2word[&6], five[0]) == 1 {
            number2word.insert(5, five[0]);
            word2number.insert(five[0].clone(), 5);
            number2word.insert(2, five[1]);
            word2number.insert(five[1].clone(), 2);
        } else {
            number2word.insert(5, five[1]);
            word2number.insert(five[1].clone(), 5);
            number2word.insert(2, five[0]);
            word2number.insert(five[0].clone(), 2);
        }
        let mut now = 0;
        for output in &outputs[i] {
            now *= 10;
            now += word2number[output];
        }
        ans += now;
    }
    dbg!(ans);
    Ok(())
}
fn dif(word1: &String, word2: &String) -> usize {
    let set1: HashSet<_> = word1.chars().collect();
    let set2: HashSet<_> = word2.chars().collect();
    set1.symmetric_difference(&set2).count()
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
