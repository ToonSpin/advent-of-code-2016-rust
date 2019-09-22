use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::separated_list;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;
use std::io;
use std::io::prelude::*;

type Screen = [[bool; 50]; 6];

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    fn execute(&self, mut screen: Screen) -> Screen {
        match self {
            Instruction::Rect(w, h) => {
                for y in 0..*h {
                    for x in 0..*w {
                        screen[y][x] = true;
                    }
                }
            }
            Instruction::RotateRow(y, n) => {
                let mut new_row = [false; 50];
                for i in 0..50 {
                    new_row[(i + *n) % 50] = screen[*y][i];
                }
                screen[*y] = new_row;
            }
            Instruction::RotateColumn(x, n) => {
                let mut new_col = [false; 6];
                for i in 0..6 {
                    new_col[(i + *n) % 6] = screen[i][*x];
                }
                for i in 0..6 {
                    screen[i][*x] = new_col[i];
                }
            }
        }
        screen
    }
}

fn print_screen(screen: &Screen) {
    for row in screen.iter() {
        let s: String = row.iter().map(|b| if *b { '#' } else { ' ' }).collect();
        println!("{}", s);
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_rect(input: &str) -> IResult<&str, Instruction> {
    let parser = separated_pair(parse_usize, char('x'), parse_usize);
    let parser = preceded(tag("rect "), parser);
    map(parser, |(a, b)| Instruction::Rect(a, b))(input)
}

fn parse_rotate_row(input: &str) -> IResult<&str, Instruction> {
    let parser = separated_pair(parse_usize, tag(" by "), parse_usize);
    let parser = preceded(tag("rotate row y="), parser);
    map(parser, |(a, b)| Instruction::RotateRow(a, b))(input)
}

fn parse_rotate_column(input: &str) -> IResult<&str, Instruction> {
    let parser = separated_pair(parse_usize, tag(" by "), parse_usize);
    let parser = preceded(tag("rotate column x="), parser);
    map(parser, |(a, b)| Instruction::RotateColumn(a, b))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_rect, parse_rotate_row, parse_rotate_column))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list(char('\n'), parse_instruction)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_rest, input) = parse_instructions(&input[..]).unwrap();

    let mut screen = [[false; 50]; 6];

    for i in input.iter() {
        screen = i.execute(screen);
    }

    let f = |r: &[bool; 50]| r.iter().filter(|&b| *b).count();
    let sum: usize = screen.iter().map(f).sum();
    println!("The number of pixels lit on the screen: {}", sum);

    print_screen(&screen);

    Ok(())
}
