use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    x: u64,
    y: u64,
    distance: u64
}

impl Node {
    fn get_possible_coords(&self) -> Vec<(u64, u64)> {
        let mut result = Vec::new();
        if self.x > 0 {
            result.push((self.x - 1, self.y));
        }
        result.push((self.x + 1, self.y));

        if self.y > 0 {
            result.push((self.x, self.y - 1));
        }
        result.push((self.x, self.y + 1));

        result
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn is_open_space(x: u64, y: u64, favorite_number: u64) -> bool {
    let mut sum = x * (x + 3) + y * (2 * x + y + 1) + favorite_number;
    let mut bits = 0;
    while sum > 0 {
        if sum % 2 == 1 {
            bits += 1;
        }
        sum /= 2;
    }
    bits % 2 == 0
}

fn main() -> io::Result<()> {
    let mut favorite_number: u64 = 0;
    for line in io::stdin().lock().lines() {
        favorite_number = line.unwrap().parse().unwrap();
        break;
    }

    let dest_x = 31;
    let dest_y = 39;
    let max_distance = 50;

    let mut part1_done = false;
    let mut part2_done = false;
    let mut part1_distance = 0;
    let mut part2_count = 0;

    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {x: 1, y: 1, distance: 0});

    loop {
        let current_node = queue.pop().unwrap();
        let distance = current_node.distance;

        if !part1_done && current_node.x == dest_x && current_node.y == dest_y {
            part1_distance = distance;
            part1_done = true;

            if part2_done {
                break;
            }
        }

        if !part2_done && distance > max_distance {
            part2_count = visited.len();
            part2_done = true;

            if part1_done {
                break;
            }
        }

        visited.insert((current_node.x, current_node.y));
        for (x, y) in current_node.get_possible_coords() {
            if is_open_space(x, y, favorite_number) && !visited.contains(&(x, y)) {
                queue.push(Node {x, y, distance: distance + 1})
            }
        }
    }
    println!("The length of the shortest path to ({}, {}) is: {}", dest_x, dest_y, part1_distance);
    println!("The number of locations that can be visited in at most {} steps is: {}", max_distance, part2_count);

    Ok(())
}
