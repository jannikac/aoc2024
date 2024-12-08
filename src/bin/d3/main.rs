use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32},
    combinator::value,
    multi::{many0, many_till},
    sequence::{delimited, tuple},
    IResult, Parser,
};
use regex::Regex;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

/// parses ,
fn separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(",")(input)?;
    Ok((input, ()))
}

/// parses i32,i32
fn parse_multiply(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (a, _, b)) = tuple((i32, separator, i32))(input)?;
    Ok((input, (a, b)))
}

/// parses mul(i32,i32)
fn parse_mul(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mul")(i)?;
    let (i, (a, b)) = delimited(tag("("), parse_multiply, tag(")"))(i)?;
    return Ok((i, Instruction::Mul(a, b)));
}

/// parses mul(i32,i32) or don't() or do()
fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, ins) = alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_mul,
    ))(i)?;
    Ok((i, ins))
}

fn sum_of_mul(i: &'static str) -> anyhow::Result<()> {
    let (i, r) = many0(many_till(anychar, parse_instruction).map(|v| v.1))(i).context("failure")?;
    let (_, sum_of_mul2) = r
        .iter()
        .fold((ShouldProcess::Do, 0), |(process, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if process == ShouldProcess::Do {
                    (process, acc + a * b)
                } else {
                    (process, acc)
                }
            }
            Instruction::Do => (ShouldProcess::Do, acc),
            Instruction::Dont => (ShouldProcess::Dont, acc),
        });
    dbg!(sum_of_mul2);
    anyhow::Ok(())
}

fn part_1() {
    // reges used here could also use the parser
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let muls = re
        .captures_iter(&INPUT)
        .map(|v| (v.get(0).unwrap(), v.get(1).unwrap(), v.get(2).unwrap()))
        .collect::<Vec<_>>();
    let sum_of_muls = muls.iter().fold(0, |acc, x| {
        acc + (x.1.as_str().parse::<i32>().unwrap() * x.2.as_str().parse::<i32>().unwrap())
    });
    dbg!(sum_of_muls);
}

fn part_2() {
    sum_of_mul(&INPUT).unwrap();
}

fn main() {
    part_1();
    part_2();
}
