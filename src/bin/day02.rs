use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;
use std::io;
use std::io::prelude::*;

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}

fn get_bathroom_code(
    input: &Vec<Vec<Direction>>,
    start_pos: (usize, usize),
    keypad: &Keypad,
) -> std::string::String {
    let mut bathroom_code = String::new();
    let mut pos = start_pos;

    for instruction_set in input.iter() {
        for direction in instruction_set.iter() {
            let new_pos = match direction {
                Direction::Up => (pos.0 - 1, pos.1),
                Direction::Down => (pos.0 + 1, pos.1),
                Direction::Left => (pos.0, pos.1 - 1),
                Direction::Right => (pos.0, pos.1 + 1),
            };
            if keypad[new_pos.0][new_pos.1] != ' ' {
                pos = new_pos;
            }
        }
        bathroom_code.push_str(&format!("{}", keypad[pos.0][pos.1]));
    }

    bathroom_code
}

type Keypad = [[char; 7]; 7];

fn main() -> io::Result<()> {
    let mut input = Vec::new();

    for line in io::stdin().lock().lines() {
        let (_rest, v) = parse_directions(&line.unwrap()).unwrap();
        input.push(v);
    }

    let keypad1: Keypad = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', '1', '2', '3', ' ', ' ', ' '],
        [' ', '4', '5', '6', ' ', ' ', ' '],
        [' ', '7', '8', '9', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ];
    let start1 = (2, 2);

    let keypad2: Keypad = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', '1', ' ', ' ', ' '],
        [' ', ' ', '2', '3', '4', ' ', ' '],
        [' ', '5', '6', '7', '8', '9', ' '],
        [' ', ' ', 'A', 'B', 'C', ' ', ' '],
        [' ', ' ', ' ', 'D', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ];
    let start2 = (3, 2);

    println!(
        "The bathroom code (part 1): {}",
        get_bathroom_code(&input, start1, &keypad1)
    );
    println!(
        "The bathroom code (part 2): {}",
        get_bathroom_code(&input, start2, &keypad2)
    );

    Ok(())
}
