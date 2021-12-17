use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
fn main() -> Result<()> {
    let f = BufReader::new(File::open("input")?);
    let mut map = Vec::new();
    for line in f.lines() {
        let line = line.context("read line failed")?;
        map.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>(),
        )
    }
    part_one(map.clone());
    Ok(())
}

fn part_one(mut map: Vec<Vec<i32>>) {
    let mut flashes = 0;
    for _ in 0..100 {
        // for each step
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                power_up(&mut map, i, j);
            }
        }
        flashes += count_and_clear(&mut map);
    }
    dbg!(flashes);
}

fn count_and_clear(map: &mut [Vec<i32>]) -> i32 {
    let mut ret = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] > 9 {
                ret += 1;
                map[i][j] = 0;
            }
        }
    }
    ret
}

fn power_up(map: &mut [Vec<i32>], x: usize, y: usize) {
    if map[x][y] > 9 {
        return;
    }
    map[x][y] += 1;
    let mx = [-1i32, 0, 1, -1, 1, -1, 0, 1];
    let my = [-1, -1, -1, 0, 0, 1, 1, 1];
    if map[x][y] > 9 {
        for i in 0..8 {
            let nx = x as i32 + mx[i];
            let ny = y as i32 + my[i];
            if nx < 0 || ny < 0 || nx >= map.len() as i32 || ny >= map[nx as usize].len() as i32 {
                continue;
            }
            power_up(map, nx as usize, ny as usize);
        }
    }
}

fn pp(map: &[Vec<i32>]) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j])
        }
        println!();
    }
}
