use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character,
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
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

fn parse_mul(input: &str) -> IResult<&str, Option<Mul>> {
    let (rem, _) = take_until("mul(")(input)?;

    // always strip the "mul(" prefix so forward progress can be made
    let (rem, _) = tag("mul(")(rem)?;

    opt(tuple((
        character::complete::u32,
        tag(","),
        character::complete::u32,
        tag(")"),
    )))(rem)
    .map(|(rem, maybe)| (rem, maybe.map(|t| Mul(t.0, t.2))))
}

fn day3part1(input: String) -> anyhow::Result<u32> {
    let mut parse_many_mul = many0(parse_mul);
    //println!("{input}");
    let (_rem, parsed) = parse_many_mul(&input).map_err(|_| anyhow!("Parse error"))?;

    let parsed: Vec<Mul> = parsed.iter().filter_map(|f| *f).collect();

    //print!("{:?}", parsed);
    let result = parsed.iter().fold(0, |acc, mul| acc + mul.0 * mul.1);
    Ok(result)
}

fn day3part2(input: String) -> anyhow::Result<u32> {
    let mut parse_many_inst = many0(parse_inst);
    //println!("{input}");
    let (_rem, parsed) = parse_many_inst(&input).map_err(|e| anyhow!("{:?} Parse error", e))?;
    let parsed: Vec<Instruction> = parsed.iter().filter_map(|f| *f).collect();

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Do,
    Dont,
    Mul(Mul),
}

fn parse_inst(input: &str) -> IResult<&str, Option<Instruction>> {
    let result: IResult<&str, &str> = alt((tag("mul("), tag("do()"), tag("don't()")))(input);

    match result {
        // move forward a character if none of the tags matched..
        Err(_e) => map(take(1 as u32), |_| None)(input),
        Ok((rem, inst)) => match inst {
            "do()" => Ok((rem, Some(Instruction::Do))),
            "don't()" => Ok((rem, Some(Instruction::Dont))),
            _ => {
                let result: IResult<&str, (u32, &str, u32, &str)> = tuple((
                    character::complete::u32,
                    tag(","),
                    character::complete::u32,
                    tag(")"),
                ))(rem);

                match result {
                    Err(_) => Ok((rem, None)),
                    Ok((rem, mul)) => Ok((rem, Some(Instruction::Mul(Mul(mul.0, mul.2))))),
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_mul() {
        let expected = Some(Mul(764, 406));
        let (rem, mul) = parse_mul("mul(764,406)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(mul, expected);

        let (rem, mul) = parse_mul("asadmul(764,406)asdas").unwrap();
        assert_eq!("asdas", rem);
        assert_eq!(mul, expected);

        let (rem, mul) = parse_mul("mul(BAD,123)asadmul(764,406)asdas").unwrap();
        assert_eq!(mul, None);
        assert_eq!(rem, "BAD,123)asadmul(764,406)asdas");
        let (rem, mul) = parse_mul(rem).unwrap();
        assert_eq!("asdas", rem);
        assert_eq!(mul, expected);
    }

    #[test]
    pub fn test_parse_inst() {
        let expected = Some(Instruction::Mul(Mul(764, 406)));
        let (rem, mul) = parse_inst("mul(764,406)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(mul, expected);

        let (rem, inst) = parse_inst("do()asadmul(764,406)asdas").unwrap();
        assert_eq!("asadmul(764,406)asdas", rem);
        assert_eq!(inst, Some(Instruction::Do));
    }

    #[test]
    pub fn test_parse_file() {
        let input: String = fs::read_to_string("dat/input2.txt").unwrap();
        let mut parse_many_inst = many0(parse_inst);
        parse_many_inst(&input).unwrap();
    }
}
