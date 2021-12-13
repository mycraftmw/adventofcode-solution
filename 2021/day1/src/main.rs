use std::{fs::File, io::Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let levels = content
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|st| st.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    part1(&levels);
    part2(&levels);
    Ok(())
}

fn part2(levels: &Vec<i32>) {
    let mut ans = 0;
    let mut upper = levels[0] + levels[1] + levels[2];
    for i in 1..levels.len() - 2 {
        let lower = upper - levels[i - 1] + levels[i + 2];
        if lower > upper {
            ans += 1
        }
        upper = lower;
    }
    println!("part 2: {}", ans)
}

fn part1(levels: &Vec<i32>) {
    let mut ans = 0;
    for i in 1..levels.len() {
        if levels[i] > levels[i - 1] {
            ans += 1
        }
    }
    println!("part 1: {}", ans);
}
