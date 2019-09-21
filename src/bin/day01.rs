use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize, value},
    multi::separated_list,
    sequence::pair,
    IResult,
};

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

struct Instruction {
    dir: Direction,
    dist: i32,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(input)
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| s.parse())(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let parser = pair(parse_direction, parse_i32);
    map(parser, |(dir, dist)| Instruction { dir, dist })(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list(tag(", "), parse_instruction)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_rest, input) = parse_instructions(input).unwrap();

    let mut dir_vec = (0, 1);
    let mut pos = (0, 0);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);
    let mut pos_part2: Option<(i32, i32)> = None;

    for i in input.iter() {
        match i.dir {
            Direction::Left => dir_vec = (-dir_vec.1, dir_vec.0),
            Direction::Right => dir_vec = (dir_vec.1, -dir_vec.0),
        }
        if let None = pos_part2 {
            let mut pos_path = pos;
            for _i in 0..i.dist {
                pos_path.0 += dir_vec.0;
                pos_path.1 += dir_vec.1;
                if visited.insert(pos_path) == false {
                    pos_part2 = Some(pos_path);
                    break;
                }
            }
        }
        pos.0 += dir_vec.0 * i.dist;
        pos.1 += dir_vec.1 * i.dist;
    }

    println!(
        "The distance to the final position: {}",
        pos.0.abs() + pos.1.abs()
    );
    let pos_part2 = pos_part2.unwrap();
    println!(
        "The distance to the Easter Bunny Headquarters: {}",
        pos_part2.0.abs() + pos_part2.1.abs()
    );

    Ok(())
}
