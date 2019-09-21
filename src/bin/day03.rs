use std::io;
use std::io::prelude::*;

use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    multi::{many0, separated_list},
    sequence::{preceded, tuple},
    IResult,
};

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    preceded(many0(char(' ')), parse_u32)(input)
}

fn parse_triplet(input: &str) -> IResult<&str, (u32, u32, u32)> {
    tuple((parse_number, parse_number, parse_number))(input)
}

fn parse_triplets(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    separated_list(char('\n'), parse_triplet)(input)
}

fn is_valid_triangle(t: &(u32, u32, u32)) -> bool {
    t.0 + t.1 > t.2 && t.0 + t.2 > t.1 && t.1 + t.2 > t.0
}

struct Part2Iterator<'a> {
    input: &'a Vec<(u32, u32, u32)>,
    pos: usize,
}

impl<'a> Part2Iterator<'a> {
    fn new(input: &'a Vec<(u32, u32, u32)>) -> Part2Iterator {
        Part2Iterator { input, pos: 0 }
    }
}

impl<'a> Iterator for Part2Iterator<'a> {
    type Item = (u32, u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            None
        } else {
            let r = 3 * (self.pos / 3);
            let triplet = match self.pos % 3 {
                0 => (self.input[r].0, self.input[r + 1].0, self.input[r + 2].0),
                1 => (self.input[r].1, self.input[r + 1].1, self.input[r + 2].1),
                2 => (self.input[r].2, self.input[r + 1].2, self.input[r + 2].2),
                _ => unreachable!(),
            };
            self.pos += 1;
            Some(triplet)
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_rest, input) = parse_triplets(&input[..]).unwrap();

    let iter1 = input.iter();
    let iter2 = Part2Iterator::new(&input);

    println!(
        "The number of valid triangles in the input: {}",
        iter1.filter(|&t| is_valid_triangle(t)).count()
    );
    println!(
        "The number of valid triangles in the input: {}",
        iter2.filter(|&t| is_valid_triangle(&t)).count()
    );

    Ok(())
}
