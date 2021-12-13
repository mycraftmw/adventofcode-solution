use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let mut f = BufReader::new(File::open("input.txt")?);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let inits: Vec<i32> = buf
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let days = 256;
    let mut f = vec![vec![0i64; 9]; days + 1];
    for j in 0..9 {
        f[0][j] = 1;
    }
    for i in 1..=days {
        for j in 0..=8 {
            if j == 0 {
                f[i][j] = f[i - 1][6] + f[i - 1][8];
            } else {
                f[i][j] = f[i - 1][j - 1];
            }
        }
    }
    let ans = inits.iter().fold(0, |acc, x| acc + f[days][*x as usize]);
    dbg!(ans);
    Ok(())
}
