use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use anyhow::{Context, Result};
fn main() -> Result<()> {
    let mut f = BufReader::new(File::open("input.txt")?);
    let mut buf = String::new();
    let mut map = Vec::new();
    while f.read_line(&mut buf)? > 0 {
        let row: Vec<i32> = buf
            .trim()
            .chars()
            .collect::<Vec<_>>()
            .iter()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        map.push(row);
        buf.clear();
    }
    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut flag = true;
            if i > 0 && map[i][j] >= map[i - 1][j] {
                flag = false;
            }
            if i + 1 < map.len() && map[i][j] >= map[i + 1][j] {
                flag = false;
            }
            if j > 0 && map[i][j] >= map[i][j - 1] {
                flag = false;
            }
            if j + 1 < map[i].len() && map[i][j] >= map[i][j + 1] {
                flag = false;
            }
            if flag {
                ans += 1 + map[i][j];
            }
        }
    }
    dbg!(ans);
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut sizes = BinaryHeap::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let size = visit(&map, &mut visited, i, j)?;
            sizes.push(size);
        }
    }
    let a = sizes.pop().context("not enough data")?;
    let b = sizes.pop().context("not enough data")?;
    let c = sizes.pop().context("not enough data")?;
    dbg!(a * b * c);
    Ok(())
}

fn visit(map: &[Vec<i32>], visited: &mut [Vec<bool>], x: usize, y: usize) -> Result<i32> {
    if visited[x][y] || map[x][y] == 9 {
        return Ok(0);
    }
    visited[x][y] = true;
    let mx = [0, 1, 0, -1];
    let my = [1, 0, -1, 0];
    let mut res = 1;
    for i in 0..4 {
        let nx = x as i32 + mx[i];
        let ny = y as i32 + my[i];
        if nx < 0 || ny < 0 || nx >= map.len() as i32 || ny >= map[nx as usize].len() as i32 {
            continue;
        }
        if map[nx as usize][ny as usize] != 9 {
            res += visit(map, visited, nx as usize, ny as usize)?;
        }
    }
    Ok(res)
}
