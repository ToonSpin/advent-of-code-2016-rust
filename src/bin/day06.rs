use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::character::complete::{alpha1, char};
use nom::multi::separated_list;
use nom::IResult;

fn parse_message(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_messages(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list(char('\n'), parse_message)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_rest, input) = parse_messages(&input[..]).unwrap();

    let mut character_counts: Vec<HashMap<char, u32>> = Vec::new();

    for _i in 0..input[0].len() {
        character_counts.push(HashMap::new());
    }

    for message in input.iter() {
        for (i, c) in message.chars().enumerate() {
            let count = character_counts[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    let mut message_part1 = String::new();
    let mut message_part2 = String::new();

    for counts in character_counts.iter() {
        let (c, _n) = counts.iter().max_by_key(|(_c, n)| *n).unwrap();
        message_part1.push(*c);
        let (c, _n) = counts.iter().min_by_key(|(_c, n)| *n).unwrap();
        message_part2.push(*c);
    }

    println!("The message is (part 1): {}", message_part1);
    println!("The message is (part 2): {}", message_part2);

    Ok(())
}
