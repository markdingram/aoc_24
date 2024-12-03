use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character,
    combinator::{map, value},
    multi::{many0, many_till},
    sequence::tuple,
    IResult, Parser as _,
};
use std::{fs, io::Write as _};

wasi::cli::command::export!(Day3);

struct Day3;

impl wasi::exports::cli::run::Guest for Day3 {
    fn run() -> Result<(), ()> {
        let mut stdout = wasi::cli::stdout::get_stdout();

        let input: String = fs::read_to_string("dat/input.txt").unwrap();
        let r = day3part1(input).unwrap();
        stdout
            .write_all(format!("Day 3 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        let input: String = fs::read_to_string("dat/input.txt").unwrap();
        let r = day3part2(input).unwrap();
        stdout
            .write_all(format!("Day 3 Pt 2 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mul(u32, u32);

/// For part 2 there can be different types of "Instruction"
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Do,
    Dont,
    Mul(Mul),
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let val = tuple((
        tag("mul("),
        character::complete::u32,
        tag(","),
        character::complete::u32,
        tag(")"),
    ));

    map(val, |(_, i, _, j, _)| Mul(i, j))(input)
}

fn parse_next_mul(input: &str) -> IResult<&str, Mul> {
    let parser = many_till(take(1usize), parse_mul);
    map(parser, |(_, str)| str).parse(input)
}

fn day3part1(input: String) -> anyhow::Result<u32> {
    let mut parse_many_mul = many0(parse_next_mul);
    //println!("{input}");
    let (_rem, parsed) = parse_many_mul(&input).map_err(|_| anyhow!("Parse error"))?;

    //print!("{:?}", parsed);
    let result = parsed.iter().fold(0, |acc, mul| acc + mul.0 * mul.1);
    Ok(result)
}

fn day3part2(input: String) -> anyhow::Result<u32> {
    let mut parse_many_inst = many0(parse_next_inst);
    let (_rem, parsed) = parse_many_inst(&input).map_err(|e| anyhow!("{:?} Parse error", e))?;

    //println!("{:?}", parsed);
    let result = parsed
        .iter()
        .fold((true, 0), |(on, result), inst| match inst {
            Instruction::Do => (true, result),
            Instruction::Dont => (false, result),
            Instruction::Mul(mul) => (on, if on { result + mul.0 * mul.1 } else { result }),
        });
    Ok(result.1)
}

/// Parses an Inst from the input
fn parse_inst(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
        map(parse_mul, |m| Instruction::Mul(m)),
    ))(input)
}

/// Skips forward 1 char at a time until an inst was parsed
fn parse_next_inst(input: &str) -> IResult<&str, Instruction> {
    let parser = many_till(take(1usize), parse_inst);
    map(parser, |(_, str)| str).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_mul() {
        let expected = Mul(764, 406);
        let (rem, mul) = parse_mul("mul(764,406)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(mul, expected);

        let (rem, mul) = parse_next_mul("asadmul(764,406)asdas").unwrap();
        assert_eq!("asdas", rem);
        assert_eq!(mul, expected);

        let (rem, mul) = parse_next_mul("mul(BAD,123)asadmul(764,406)asdas").unwrap();
        assert_eq!("asdas", rem);
        assert_eq!(mul, expected);
    }

    #[test]
    pub fn test_parse_inst() {
        let expected = Instruction::Mul(Mul(764, 406));
        let (rem, mul) = parse_next_inst("mul(764,406)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(mul, expected);

        let (rem, inst) = parse_inst("do()asadmul(764,406)asdas").unwrap();
        assert_eq!("asadmul(764,406)asdas", rem);
        assert_eq!(inst, Instruction::Do);
    }
}
