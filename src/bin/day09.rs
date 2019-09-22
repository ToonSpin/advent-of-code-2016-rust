use std::io;
use std::io::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{char, digit1, none_of},
    combinator::{map, map_parser, map_res},
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

enum Token {
    Char,
    Marker(u64, u64, Vec<Token>),
}

impl Token {
    fn len_part1(&self) -> u64 {
        match self {
            Token::Char => 1,
            Token::Marker(l, r, _v) => l * r,
        }
    }

    fn len_part2(&self) -> u64 {
        match self {
            Token::Char => 1,
            Token::Marker(_l, r, v) => r * Self::vec_len_part2(&v),
        }
    }

    fn vec_len_part1(v: &Vec<Token>) -> u64 {
        v.iter().map(|t| t.len_part1()).sum()
    }

    fn vec_len_part2(v: &Vec<Token>) -> u64 {
        v.iter().map(|t| t.len_part2()).sum()
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_marker(input: &str) -> IResult<&str, Token> {
    let parser = separated_pair(parse_u64, char('x'), parse_u64);
    let (rest, (l, r)) = delimited(char('('), parser, char(')'))(input)?;
    let (rest, tokens) = map_parser(take(l), parse_tokens)(rest)?;
    Ok((rest, Token::Marker(l, r, tokens)))
}

fn parse_char(input: &str) -> IResult<&str, Token> {
    map(none_of("("), |_c| Token::Char)(input)
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((parse_char, parse_marker))(input)
}

fn parse_tokens(input: &str) -> IResult<&str, Vec<Token>> {
    many1(parse_token)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = parse_tokens(input).unwrap();

    println!(
        "Decompressed length (part 1): {}",
        Token::vec_len_part1(&input)
    );
    println!(
        "Decompressed length (part 2): {}",
        Token::vec_len_part2(&input)
    );

    Ok(())
}
