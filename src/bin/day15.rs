use std::io;
use std::io::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list,
    IResult,
    sequence::{delimited, preceded, tuple}
};

#[derive(Debug)]
struct Disc {
    id: u32,
    positions: u32,
    start_position: u32
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    preceded(tag("Disc #"), parse_u32)(input)
}

fn parse_positions(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has "), parse_u32)(input)
}

fn parse_start_position(input: &str) -> IResult<&str, u32> {
    delimited(
        tag(" positions; at time=0, it is at position "),
        parse_u32,
        tag(".")
    )(input)
}

fn parse_discs(input: &str) -> IResult<&str, Vec<Disc>> {
    separated_list(tag("\n"), parse_disc)(input)
}

fn parse_disc(input: &str) -> IResult<&str, Disc> {
    let (rest, (id, positions, start_position)) = tuple((
        parse_id,
        parse_positions,
        parse_start_position
    ))(input)?;
    Ok((rest, Disc {id, positions, start_position}))
}

fn timestamp_till_first_capsule(discs: &Vec<Disc>) -> u32 {
    let mut time = 0;
    while !can_get_capsule(time, &discs) {
        time += 1;
    }
    time
}

fn can_get_capsule(time: u32, discs: &Vec<Disc>) -> bool {
    for (i, disc) in discs.iter().enumerate() {
        if (disc.start_position + time + i as u32 + 1) % disc.positions != 0 {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_rest, mut input) = parse_discs(input).unwrap();

    println!("Time until the first capsule: {}", timestamp_till_first_capsule(&input));

    input.push(Disc {id: input.len() as u32 + 1, positions: 11, start_position: 0});
    println!("Time until the first capsule with the extra disc: {}", timestamp_till_first_capsule(&input));

    Ok(())
}
