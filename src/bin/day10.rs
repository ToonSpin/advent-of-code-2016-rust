use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
enum OutputType {
    Bot(u32),
    Output(u32),
}

#[derive(Debug)]
struct Instruction {
    bot_actor: u32,
    output_low: OutputType,
    output_high: OutputType,
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let input_bin_regex = r"^value (\d+) goes to bot (\d+)";
    let instruction_regex = r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)";

    let re_input_bin = Regex::new(input_bin_regex).unwrap();
    let re_instruction = Regex::new(instruction_regex).unwrap();

    let mut bot_values: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut outputs: HashMap<u32, u32> = HashMap::new();
    let mut bot_instructions: HashMap<u32, Instruction> = HashMap::new();

    for line in input.split("\n") {
        if re_input_bin.is_match(line) {
            let caps = re_input_bin.captures(line).unwrap();
            let value = caps.get(1).unwrap().as_str().parse().unwrap();
            let bot = caps.get(2).unwrap().as_str().parse().unwrap();
            bot_values.entry(bot).or_insert(Vec::new()).push(value);
        }
        if re_instruction.is_match(line) {
            let caps = re_instruction.captures(line).unwrap();

            let bot_actor = caps.get(1).unwrap().as_str().parse().unwrap();
            let out_low: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
            let out_high: u32 = caps.get(5).unwrap().as_str().parse().unwrap();

            let output_low = match caps.get(2).unwrap().as_str() {
                "bot" => OutputType::Bot(out_low),
                "output" => OutputType::Output(out_low),
                _ => unreachable!(),
            };

            let output_high = match caps.get(4).unwrap().as_str() {
                "bot" => OutputType::Bot(out_high),
                "output" => OutputType::Output(out_high),
                _ => unreachable!(),
            };

            let instr = Instruction {
                bot_actor,
                output_low,
                output_high,
            };
            bot_instructions.insert(bot_actor, instr);
        }
    }

    loop {
        let next_bot = bot_values.iter().filter(|(_k, v)| v.len() > 1).next();
        if let None = next_bot {
            break;
        }

        let (bot, values) = next_bot.unwrap();
        assert_eq!(values.len(), 2);

        let low;
        let high;

        if values[0] < values[1] {
            low = values[0];
            high = values[1];
        } else {
            low = values[1];
            high = values[0];
        }

        if low == 17 && high == 61 {
            println!("The bot responsible for comparing 17 to 61 is: {}", bot);
        }

        let instr = bot_instructions.get(bot).unwrap();
        match instr.output_low {
            OutputType::Bot(value) => {
                bot_values.entry(value).or_insert(Vec::new()).push(low);
            }
            OutputType::Output(value) => {
                outputs.insert(value, low);
            }
        }
        match instr.output_high {
            OutputType::Bot(value) => {
                bot_values.entry(value).or_insert(Vec::new()).push(high);
            }
            OutputType::Output(value) => {
                outputs.insert(value, high);
            }
        }
        bot_values.insert(instr.bot_actor, Vec::new());
    }

    let mut answer: u32 = *outputs.get(&0).unwrap();
    answer *= outputs.get(&1).unwrap();
    answer *= outputs.get(&2).unwrap();
    println!("Multiplying output bins 0, 1, and 2 gives: {}", answer);

    Ok(())
}
