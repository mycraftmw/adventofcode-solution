use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

fn main() -> anyhow::Result<()> {
    let f = BufReader::new(File::open("input.txt")?);
    let mut lines = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x1, y1): (usize, usize) = (x1.parse().unwrap(), y1.parse().unwrap());
        let (x2, y2) = p2.split_once(",").unwrap();
        let (x2, y2): (usize, usize) = (x2.parse().unwrap(), y2.parse().unwrap());
        max_x = max_x.max(x1);
        max_x = max_x.max(x2);
        max_y = max_y.max(y1);
        max_y = max_y.max(y2);
        lines.push(if less(x1, y1, x2, y2) {
            Line { x1, y1, x2, y2 }
        } else {
            Line {
                x1: x2,
                y1: y2,
                x2: x1,
                y2: y1,
            }
        });
    }
    lines.sort_by(|a, b| {
        if a.x1 != b.x1 {
            a.x1.cmp(&b.x1)
        } else {
            a.y1.cmp(&b.y1)
        }
    });
    let mut board = vec![vec![0; max_x + 1]; max_y + 1];
    part_one(&mut lines, &mut board);
    let mut board = vec![vec![0; max_x + 1]; max_y + 1];
    part_two(&mut lines, &mut board);
    Ok(())
}

fn part_one(lines: &[Line], board: &mut Vec<Vec<i32>>) {
    let mut ans = 0;
    for line in lines {
        if line.x1 == line.x2 && line.y1 == line.y2 {
            board[line.y1][line.x1] += 1;
            if board[line.y1][line.x1] == 2 {
                ans += 1;
            }
            continue;
        }
        if line.x1 == line.x2 {
            for y in line.y1..=line.y2 {
                board[y][line.x1] += 1;
                if board[y][line.x1] == 2 {
                    ans += 1;
                }
            }
        }
        if line.y1 == line.y2 {
            for x in line.x1..=line.x2 {
                board[line.y1][x] += 1;
                if board[line.y1][x] == 2 {
                    ans += 1;
                }
            }
        }
    }
    dbg!(ans);
}

fn part_two(lines: &[Line], board: &mut Vec<Vec<i32>>) {
    let mut ans = 0;
    for line in lines {
        if line.x1 == line.x2 && line.y1 == line.y2 {
            board[line.y1][line.x1] += 1;
            if board[line.y1][line.x1] == 2 {
                ans += 1;
            }
            continue;
        }
        if line.x1 == line.x2 {
            for y in line.y1..=line.y2 {
                board[y][line.x1] += 1;
                if board[y][line.x1] == 2 {
                    ans += 1;
                }
            }
            continue;
        }
        if line.y1 == line.y2 {
            for x in line.x1..=line.x2 {
                board[line.y1][x] += 1;
                if board[line.y1][x] == 2 {
                    ans += 1;
                }
            }
            continue;
        }
        for x in line.x1..=line.x2 {
            let y =
                line.y1 as i32 + (x - line.x1) as i32 * (if line.y1 < line.y2 { 1 } else { -1 });
            board[y as usize][x] += 1;
            if board[y as usize][x] == 2 {
                ans += 1;
            }
        }
    }
    dbg!(ans);
}

fn pp(board: &[Vec<i32>]) {
    for row in board {
        for c in row {
            print!("{},", c);
        }
        println!();
    }
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn less(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    if x1 != x2 {
        x1 < x2
    } else {
        y1 <= y2
    }
}
