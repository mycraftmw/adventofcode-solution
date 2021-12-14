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
    part_two(&positions);
    Ok(())
}

fn part_two(positions: &[i32]) {
    let sum = positions.iter().fold(0, |acc, x| acc + x);
    let init = sum as f64 / positions.len() as f64;
    let p1 = (init + 0.5).round();
    let p2 = (init - 0.5).round();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in positions {
        ans1 += fuel(*i as i64 - p1 as i64);
        ans2 += fuel(*i as i64 - p2 as i64);
        // dbg!(i, ans1);
    }
    dbg!(ans1, ans2);
    dbg!(ans1.min(ans2));
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

fn fuel(dis: i64) -> i64 {
    let mut dis = dis;
    if dis < 0 {
        dis = -dis;
    }
    (1 + dis) * dis / 2
}
