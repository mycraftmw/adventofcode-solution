use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let mut f = BufReader::new(File::open("input.txt")?);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let mut positions: Vec<i32> = buf
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    positions.sort();
    part_one(&positions);
    Ok(())
}

fn part_one(positions: &[i32]) {
    let mut sum = positions.iter().fold(0, |acc, x| acc + x - positions[0]);
    let mut ans = sum;
    let mut mid = 0;
    for i in 0..positions.len() {
        if positions[i] == positions[mid] {
            continue;
        }
        let dif = positions[i] - positions[mid];
        sum = sum + i as i32 * dif - (positions.len() - i) as i32 * dif;
        mid = i;
        ans = ans.min(sum);
    }
    dbg!(ans);
}
