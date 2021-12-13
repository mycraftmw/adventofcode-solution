use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let mut f = BufReader::new(File::open("input.txt")?);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let chosen_numbers: Vec<i32> = buf
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let number_index: HashMap<i32, i32> = chosen_numbers
        .iter()
        .enumerate()
        .map(|(i, e)| (*e, i as i32))
        .collect();
    let mut boards = Vec::new();
    let mut scores = Vec::new();

    while f.read_line(&mut buf)? > 0 {
        buf.clear();
        let mut board = Vec::new();
        let mut score = Vec::new();
        for _ in 0..5 {
            f.read_line(&mut buf)?;
            let row: Vec<i32> = buf
                .trim()
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();
            let s_row: Vec<i32> = row.iter().map(|i| number_index[i]).collect();
            score.push(s_row);
            board.push(row);
            buf.clear();
        }
        boards.push(board);
        scores.push(score);
    }
    // start calculating part one
    let mut ans = 0;
    let mut minimal_max = chosen_numbers.len() as i32;
    for idx in 0..boards.len() {
        // for each row
        for r in 0..5 {
            let last = scores[idx][r].iter().fold(0, |max, x| max.max(*x));
            if last < minimal_max {
                minimal_max = last;
                let left = sum(&boards[idx], &number_index, last);
                ans = left * chosen_numbers[last as usize];
                // dbg!(&minimal_max, &left, &chosen_numbers[last as usize], ans);
            }
        }
        // for each collumn
        for c in 0..5 {
            let last = (0..5).fold(0, |max, x| max.max(scores[idx][x][c]));
            if last < minimal_max {
                minimal_max = last;
                let left = sum(&boards[idx], &number_index, last);
                ans = left * chosen_numbers[last as usize];
                // dbg!(&minimal_max, &left, &chosen_numbers[last as usize], ans);
            }
        }
    }
    println!("{}", ans);
    // start calculating part two
    let mut maximal_max = 0;
    let mut final_ans = 0;
    for idx in 0..boards.len() {
        let mut this_minimal = chosen_numbers.len() as i32;
        // each row
        for r in 0..5 {
            let last = scores[idx][r].iter().fold(0, |max, x| max.max(*x));
            if last < this_minimal {
                this_minimal = last;
                let left = sum(&boards[idx], &number_index, last);
                ans = left * chosen_numbers[last as usize];
            }
        }
        // each col
        for c in 0..5 {
            let last = (0..5).fold(0, |max, x| max.max(scores[idx][x][c]));
            if last < this_minimal {
                this_minimal = last;
                let left = sum(&boards[idx], &number_index, last);
                ans = left * chosen_numbers[last as usize];
            }
        }
        if this_minimal > maximal_max {
            maximal_max = this_minimal;
            final_ans = ans;
        }
    }
    println!("{}", final_ans);
    Ok(())
}

fn sum(board: &Vec<Vec<i32>>, number_index: &HashMap<i32, i32>, last: i32) -> i32 {
    board.iter().fold(0, |acc, each_row| {
        acc + each_row.iter().fold(0, |acc, x| {
            if number_index[x] <= last {
                acc
            } else {
                acc + x
            }
        })
    })
}
