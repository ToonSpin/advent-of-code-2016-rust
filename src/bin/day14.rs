use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

struct Stream<'a> {
    salt: &'a str,
    memo: HashMap<String, String>,
    memo_quintuplets: HashMap<(u64, u8), bool>,
    memo_triplets: HashMap<u64, Option<u8>>
}

impl<'a> Stream<'a> {
    fn new(salt: &'a str) -> Self {
        Stream {
            salt,
            memo: HashMap::new(),
            memo_quintuplets: HashMap::new(),
            memo_triplets: HashMap::new()
        }
    }

    fn get_hash_stretched_memoized(&mut self, input: String) -> String {
        if !self.memo.contains_key(&input) {
            let mut hash: String = String::from(format!("{:x}", md5::compute(input.clone())));
            for _i in 0..2016 {
                hash = String::from(format!("{:x}", md5::compute(hash)));
            }
            self.memo.insert(input.to_string(), hash);
        }
        self.memo.get(&input).unwrap().to_string()
    }

    fn get_hash_stretched(&mut self, n: u64) -> Vec<u8> {
        let input = format!("{}{}", self.salt, n);
        let hash: String = self.get_hash_stretched_memoized(input);
        String::from(hash).into_bytes()
    }

    fn get_hash(&self, n: u64) -> Vec<u8> {
        let input = format!("{}{}", self.salt, n);
        let hash = format!("{:x}", md5::compute(input));
        String::from(hash).into_bytes()
    }

    fn contains_same_sequence_specified(hash: &Vec<u8>, length: usize, character: u8) -> bool {
        'outerloop: for i in 0..hash.len() - length + 1 {
            for j in 0..length {
                if hash[i + j] != character {
                    continue 'outerloop;
                } 
            }
            return true;
        }
        false
    }

    fn contains_same_sequence(hash: &Vec<u8>, length: usize) -> Option<u8> {
        'outerloop: for i in 0..hash.len() - length + 1 {
            let character = hash[i];
            for j in 1..length {
                if hash[i + j] != character {
                    continue 'outerloop;
                }
            }
            return Some(character);
        }
        None
    }

    fn is_key_part_one(&self, hash_index: u64) -> bool {
        let hash = self.get_hash(hash_index);
        if let Some(c) = Self::contains_same_sequence(&hash, 3) {
            for next_index in hash_index + 1..=hash_index + 1000 {
                let hash = self.get_hash(next_index);
                if Self::contains_same_sequence_specified(&hash, 5, c) {
                    return true;
                }
            }
        }
        false
    }

    fn contains_quintuplet(&mut self, index: u64, character: u8) -> bool {
        if self.memo_quintuplets.contains_key(&(index, character)) {
            *self.memo_quintuplets.get(&(index, character)).unwrap()
        } else {
            let hash = self.get_hash_stretched(index);
            let contains_quintuplet = Self::contains_same_sequence_specified(&hash, 5, character);
            self.memo_quintuplets.insert((index, character), contains_quintuplet);
            contains_quintuplet
        }
    }

    fn contains_triplet(&mut self, index: u64) -> Option<u8> {
        if self.memo_triplets.contains_key(&index) {
            *self.memo_triplets.get(&index).unwrap()
        } else {
            let hash = self.get_hash_stretched(index);
            let contains_triplet = Self::contains_same_sequence(&hash, 3);
            self.memo_triplets.insert(index, contains_triplet);
            contains_triplet
        }
    }

    fn is_key_part_two(&mut self, hash_index: u64) -> bool {
        if let Some(c) = self.contains_triplet(hash_index) {
            for next_index in hash_index + 1..=hash_index + 1000 {
                if self.contains_quintuplet(next_index, c) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let mut stream = Stream::new(input);

    let mut part1_done = false;
    let mut count_part1 = 0;
    let mut index_part1 = 0;

    let mut part2_done = false;
    let mut count_part2 = 0;
    let mut index_part2 = 0;

    let mut index = 0;

    loop {
        if !part1_done && stream.is_key_part_one(index) {
            count_part1 += 1;
            if count_part1 == 64 {
                part1_done = true;
                index_part1 = index;
                if part2_done {
                    break;
                }
            }
        }
        if !part2_done && stream.is_key_part_two(index) {
            count_part2 += 1;
            if count_part2 == 64 {
                part2_done = true;
                index_part2 = index;
                if part1_done {
                    break;
                }
            }
        }
        index += 1;
    }

    println!("The index of the 64th key is: {}", index_part1);
    println!("The index of the 64th stretched key is: {}", index_part2);

    Ok(())
}
