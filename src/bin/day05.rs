use std::io;
use std::io::prelude::*;

fn nibble(hash: &[u8; 16], i: usize) -> u8 {
    let byte = hash[i / 2];
    match i % 2 {
        0 => byte / 16,
        1 => byte % 16,
        _ => unreachable!(),
    }
}

fn character(hash: &[u8; 16], i: usize) -> char {
    let characters = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    characters[nibble(&hash, i) as usize]
}

fn is_interesting(hash: &[u8; 16]) -> bool {
    hash[0] == 0 && hash[1] == 0 && nibble(hash, 4) == 0
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut password_part1 = String::new();
    let mut password_part2: Vec<Option<char>> = vec![None; 8];

    let mut part2_count = 0;
    let mut i = 0;
    while password_part1.len() < 8 || part2_count < 8 {
        let source = format!("{}{}", input, i);
        let hash = md5::compute(source);

        if is_interesting(&hash) {
            if password_part1.len() < 8 {
                password_part1.push(character(&hash, 5));
            }
            if part2_count < 8 {
                let pos = nibble(&hash, 5);
                if pos <= 7 {
                    if let None = password_part2[pos as usize] {
                        password_part2[pos as usize] = Some(character(&hash, 6));
                        part2_count += 1;
                    }
                }
            }
        }

        i += 1;
    }

    let password_part2: String = password_part2.iter().map(|c| c.unwrap()).collect();
    println!("The password (part 1): {}", password_part1);
    println!("The password (part 2): {}", password_part2);

    Ok(())
}
