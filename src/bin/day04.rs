use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    bytes::complete::take_till,
    character::complete::{alpha1, char, digit1},
    character::is_digit,
    combinator::{map, map_res},
    multi::separated_list,
    sequence::{delimited, tuple},
    IResult,
};

struct Room<'a> {
    encrypted_name: &'a str,
    sector_id: u32,
    checksum: &'a str,
    character_counts: HashMap<char, u32>,
}

impl<'a> Room<'a> {
    fn new(encrypted_name: &'a str, sector_id: u32, checksum: &'a str) -> Room<'a> {
        let mut character_counts: HashMap<char, u32> = HashMap::new();
        for c in encrypted_name.chars() {
            if c == '-' {
                continue;
            }
            let count = character_counts.entry(c).or_insert(0);
            *count += 1;
        }
        Room {
            encrypted_name,
            sector_id,
            checksum,
            character_counts,
        }
    }

    fn checksum(&self) -> String {
        let mut checksum: Vec<_> = self.character_counts.iter().collect();
        checksum.sort_by(|(c1, i1), (c2, i2)| i1.cmp(&i2).reverse().then(c1.cmp(&c2)));
        let checksum: String = checksum.into_iter().map(|(c, _i)| c).collect();
        checksum[..5].to_string()
    }

    fn is_real(&self) -> bool {
        for c in self.checksum.chars() {
            if !self.character_counts.contains_key(&c) {
                return false;
            }
        }
        self.checksum == self.checksum()
    }

    fn decrypt_char(&self, c: char) -> char {
        let i = c as u8 - b'a';
        let i = (i as u32 + self.sector_id) % 26;
        (i as u8 + b'a') as char
    }

    fn decrypt(&self) -> String {
        let mut s = String::new();
        for c in self.encrypted_name.chars() {
            s.push(match c {
                '-' => ' ',
                _ => self.decrypt_char(c),
            });
        }
        s
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_checksum(input: &str) -> IResult<&str, &str> {
    delimited(char('['), alpha1, char(']'))(input)
}

fn parse_encrypted_name<'a>(input: &'a str) -> IResult<&str, &str> {
    let f = |s: &'a str| {
        if s.len() > 0 {
            s.get(0..s.len() - 1).unwrap()
        } else {
            s
        }
    };
    map(take_till(|b| is_digit(b as u8)), f)(input)
}

fn parse_room(input: &str) -> IResult<&str, Room> {
    let parser = tuple((parse_encrypted_name, parse_u32, parse_checksum));
    let (rest, (encrypted_name, sector_id, checksum)) = parser(input)?;
    let room = Room::new(encrypted_name, sector_id, checksum);
    Ok((rest, room))
}

fn parse_rooms(input: &str) -> IResult<&str, Vec<Room>> {
    separated_list(char('\n'), parse_room)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = parse_rooms(input).unwrap();

    let sum: u32 = input
        .iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum();

    println!("The sum of the sector IDs of the real rooms: {}", sum);

    for r in input.iter() {
        if r.decrypt() == "northpole object storage" {
            println!(
                "The sector ID of the room with the objects: {}",
                r.sector_id
            );
            break;
        }
    }

    Ok(())
}
