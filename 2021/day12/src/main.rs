use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let f = BufReader::new(File::open("input.txt")?);
    let mut nodes = HashMap::new();
    let mut cap = HashMap::new();
    let mut map = HashMap::new();
    let mut node_idx = 0;
    for line in f.lines() {
        let line = line.context("read line failed")?;
        let (p1, p2) = line.split_once("-").unwrap();
        nodes.entry(p1.to_string()).or_insert_with(|| {
            node_idx += 1;
            node_idx
        });
        nodes.entry(p2.to_string()).or_insert_with(|| {
            node_idx += 1;
            node_idx
        });

        cap.insert(nodes[p1], p1 == p1.to_uppercase());
        cap.insert(nodes[p2], p2 == p2.to_uppercase());

        map.entry(nodes[p1]).or_insert(vec![]).push(nodes[p2]);
        map.entry(nodes[p2]).or_insert(vec![]).push(nodes[p1]);
    }
    let once = part_one(&nodes, &map, &cap);
    part_two(&nodes, &map, &cap, once);
    Ok(())
}

fn part_two(
    nodes: &HashMap<String, i32>,
    map: &HashMap<i32, Vec<i32>>,
    cap: &HashMap<i32, bool>,
    once: i32,
) {
    let start = nodes["start"];
    let end = nodes["end"];
    let mut token: HashMap<i32, i32> = map
        .keys()
        .map(|f| (*f, if cap[f] { -1 } else { 1 }))
        .collect();
    token.insert(start, 0);
    let mut ans = 0;
    for (_, special) in nodes {
        if *special == start || *special == end || cap[special] {
            continue;
        }
        token.insert(*special, 2);
        ans += travel2(start, map, end, &mut token, *special);
        token.insert(*special, 1);
    }
    dbg!(ans + once);
}

fn part_one(
    nodes: &HashMap<String, i32>,
    map: &HashMap<i32, Vec<i32>>,
    cap: &HashMap<i32, bool>,
) -> i32 {
    let start = nodes["start"];
    let end = nodes["end"];
    let mut token: HashMap<i32, i32> = map
        .keys()
        .map(|f| (*f, if cap[f] { -1 } else { 1 }))
        .collect();
    token.insert(start, 0);
    let ans = travel(start, map, end, &mut token);
    dbg!(ans);
    ans
}

fn travel2(
    start: i32,
    map: &HashMap<i32, Vec<i32>>,
    end: i32,
    token: &mut HashMap<i32, i32>,
    special: i32,
) -> i32 {
    if start == end {
        if token[&special] > 0 {
            return 0;
        }
        return 1;
    }
    let mut ret = 0;
    for next in &map[&start] {
        let t = token[next];
        if t != 0 {
            token.insert(*next, t - 1);
            ret += travel2(*next, map, end, token, special);
            token.insert(*next, t);
        }
    }
    ret
}

fn travel(
    start: i32,
    map: &HashMap<i32, Vec<i32>>,
    end: i32,
    token: &mut HashMap<i32, i32>,
) -> i32 {
    if start == end {
        return 1;
    }
    let mut ret = 0;
    for next in &map[&start] {
        let t = token[next];
        if t != 0 {
            token.insert(*next, t - 1);
            ret += travel(*next, map, end, token);
            token.insert(*next, t);
        }
    }
    ret
}
