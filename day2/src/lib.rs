use std::{
    cmp::max,
    fs::File,
    io::{self, BufRead as _, Write as _},
    str::FromStr,
};

wasi::cli::command::export!(Day2);

struct Day2;

impl wasi::exports::cli::run::Guest for Day2 {
    fn run() -> Result<(), ()> {
        let mut stdout = wasi::cli::stdout::get_stdout();

        let f = File::open("dat/input.txt").unwrap();
        let lines = io::BufReader::new(f).lines();
        let r = day2part1(lines).unwrap();
        stdout
            .write_all(format!("Day 2 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        let f = File::open("dat/input.txt").unwrap();
        let lines = io::BufReader::new(f).lines();
        let r = day2part2(lines).unwrap();
        stdout
            .write_all(format!("Day 2 Pt2 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        Ok(())
    }
}

pub fn day2part1(lines: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<i32> {
    let mut result = 0;

    for line in lines {
        let line: Line = line?.parse()?;

        // println!(
        //     "line: {:?} - asc: {}, desc: {}, step: {}",
        //     line.0,
        //     line.ascending(),
        //     line.descending(),
        //     line.max_step()
        // );

        if (line.ascending() || line.descending()) && line.max_step() < 4 {
            result += 1
        }
    }

    Ok(result)
}

struct Line(Vec<i32>);

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .split_whitespace()
            .map(|i| i.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Line(line))
    }
}

impl Line {
    pub fn ascending(&self) -> bool {
        self.0.is_sorted_by(|a, b| b > a)
    }

    pub fn descending(&self) -> bool {
        self.0.is_sorted_by(|a, b| b < a)
    }

    pub fn max_step(&self) -> i32 {
        self.0.windows(2).fold(0, |acc, vals| {
            max(acc, (vals.first().unwrap() - vals.last().unwrap()).abs())
        })
    }
}

pub fn day2part2(lines: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<i32> {
    let mut result = 0;

    for line in lines {
        let line: Line = line?.parse()?;

        // println!(
        //     "line: {:?} - asc: {}, desc: {}, step: {}",
        //     line.0,
        //     line.ascending(),
        //     line.descending(),
        //     line.max_step()
        // );

        if (line.ascending() || line.descending()) && line.max_step() < 4 {
            result += 1
        } else {
            // dampening
            for n in 0..line.0.len() {
                let mut dampened = line.0.clone();
                dampened.remove(n);
                let line = Line(dampened);
                if (line.ascending() || line.descending()) && line.max_step() < 4 {
                    // println!(
                    //     "dampened line: {n}, {:?} - asc: {}, desc: {}, step: {}",
                    //     line.0,
                    //     line.ascending(),
                    //     line.descending(),
                    //     line.max_step()
                    // );
                    result += 1;
                    break;
                }
            }
        }
    }

    Ok(result)
}
