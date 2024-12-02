use anyhow::anyhow;
use std::{
    fs::File,
    io::{self, BufRead as _, Write as _},
    str::FromStr,
};

wasi::cli::command::export!(Day1);

struct Day1;

impl wasi::exports::cli::run::Guest for Day1 {
    fn run() -> Result<(), ()> {
        let mut stdout = wasi::cli::stdout::get_stdout();

        let f = File::open("dat/input.txt").unwrap();
        let lines = io::BufReader::new(f).lines();
        let r = day1part1(lines).unwrap();
        stdout
            .write_all(format!("Day 1 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        let f = File::open("dat/input.txt").unwrap();
        let lines = io::BufReader::new(f).lines();
        let r = day1part2(lines).unwrap();
        stdout
            .write_all(format!("Day 1 Pt2 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        Ok(())
    }
}

pub fn day1part1(lines: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<i32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let line: Line = line?.parse()?;
        left.push(line.0);
        right.push(line.1);
    }

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    Ok(result)
}

struct Line(i32, i32);

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(char::is_whitespace)
            .ok_or_else(|| anyhow!("Bad line: {s}"))?;

        let a: i32 = a.trim().parse()?;
        let b: i32 = b.trim().parse()?;

        Ok(Line(a, b))
    }
}

pub fn day1part2(lines: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<i32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let line: Line = line?.parse()?;
        left.push(line.0);
        right.push(line.1);
    }
    right.sort();
    let mut inverted_right = right.clone();
    inverted_right.reverse();
    //println!("len: {}", left.len());

    let result = left.iter().fold(0, |acc, l| {
        let count_below = right.partition_point(|i| i < l);
        let count_above = inverted_right.partition_point(|i| i > l);
        //println!("l: {l}, count_below: {count_below}, count_above: {count_above}");

        if count_above + count_below < left.len() {
            let count = left.len() - (count_above + count_below);
            acc + l * (count as i32)
        } else {
            acc
        }
    });

    Ok(result)
}
