use std::{fs::File, io::Read};

use anyhow::Result;

fn main() -> Result<()> {
    let command_list = read_from_file("input")?;
    println!("p 1: {}", part_one(&command_list)?);
    println!("p 2: {}", part_two(&command_list)?);
    Ok(())
}

fn part_two(command_list: &[Cmd]) -> Result<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for cmd in command_list {
        match cmd.tp.as_str() {
            "down" => aim += cmd.distance,
            "up" => aim -= cmd.distance,
            "forward" => {
                x += cmd.distance;
                y += aim * cmd.distance;
            }
            _ => println!("unknown cmd"),
        }
    }
    Ok(x as i64 * y as i64)
}

fn part_one(command_list: &Vec<Cmd>) -> Result<i64> {
    let mut x = 0;
    let mut y = 0;
    for i in 0..command_list.len() {
        let c = &command_list[i];
        match c.tp.as_str() {
            "forward" => x += c.distance,
            "down" => y += c.distance,
            "up" => y -= c.distance,
            _ => println!("unknown cmd"),
        }
    }
    Ok(x as i64 * y as i64)
}

fn read_from_file(path: &str) -> Result<Vec<Cmd>> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf
        .trim()
        .split("\n")
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|each| {
            let t = each.split(" ").collect::<Vec<_>>();
            Cmd {
                tp: t[0].to_string(),
                distance: t[1].parse::<i32>().unwrap(),
            }
        })
        .collect())
}

struct Cmd {
    tp: String,
    distance: i32,
}
