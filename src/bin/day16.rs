use std::io;
use std::io::prelude::*;

fn iterate(mut data: Vec<char>) -> Vec<char> {
    let len = data.len();
    data.push('0');
    for i in 0..len {
        let character = data[len - 1 - i];
        data.push(if character == '0' { '1' } else { '0' });
    }
    data
}

fn checksum_iteration(data: &Vec<char>) -> Vec<char> {
    let mut checksum = Vec::new();
    for i in (0..data.len()).step_by(2) {
        checksum.push(if data[i] == data[i + 1] { '1' } else { '0' });
    }
    checksum
}

fn checksum(data: &Vec<char>) -> Vec<char> {
    let mut checksum = checksum_iteration(&data);
    while checksum.len() % 2 == 0 {
        checksum = checksum_iteration(&checksum);
    }
    checksum
}

fn checksum_for_disk(data: &Vec<char>, disk_size: usize) -> String {
    let mut data = data.clone();
    while data.len() < disk_size {
        data = iterate(data);
    }
    let checksum = checksum(&data[..disk_size].to_vec());
    checksum.into_iter().collect()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut data: Vec<char> = Vec::new();

    for b in input.as_bytes().iter() {
        let b = *b as char;
        if b != '0' && b != '1' {
            break;
        }
        data.push(b);
    }

    println!("The first checksum is: {}", checksum_for_disk(&data, 272));
    println!("The second checksum is: {}", checksum_for_disk(&data, 35651584));

    Ok(())
}
