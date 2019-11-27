use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{map, opt, value},
    multi::separated_list,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Clone, Debug)]
enum Item<'a> {
    Generator(&'a str),
    MicroChip(&'a str),
    Elevator,
}

type Coords = Vec<usize>;

#[derive(Debug)]
struct ItemPlanner<'a> {
    items: Vec<Item<'a>>,
    coords: Coords,
    matches: Vec<Option<usize>>,
}

#[derive(Eq)]
struct NodeDescriptor {
    coords: Coords,
    distance: usize,
}

impl NodeDescriptor {
    fn coord_sum(&self) -> usize {
        self.coords.iter().sum()
    }
    fn distance_to_dest(&self) -> usize {
        self.coords.iter().map(|c| 4 - c).sum()
    }
}

impl PartialEq for NodeDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Ord for NodeDescriptor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_dist = self.distance + self.distance_to_dest();
        let other_dist = other.distance + other.distance_to_dest();

        self_dist
            .cmp(&other_dist)
            .reverse()
            .then(self.coord_sum().cmp(&other.coord_sum()))
    }
}

impl PartialOrd for NodeDescriptor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl ItemPlanner<'_> {
    fn new(input: Vec<Vec<Item>>) -> ItemPlanner {
        let mut coords = Vec::new();
        let mut items = Vec::new();
        let mut matches = Vec::new();
        let mut element_seen = HashMap::new();
        let mut index: usize = 0;

        for (floor, floor_items) in input.iter().enumerate() {
            for item in floor_items.iter() {
                match item {
                    Item::Elevator => {
                        matches.push(None);
                    }
                    Item::Generator(e) | Item::MicroChip(e) => {
                        if element_seen.contains_key(e) {
                            let element_match: usize = *element_seen.get(e).unwrap();
                            matches[element_match] = Some(index);
                            matches.push(Some(element_match));
                        } else {
                            element_seen.insert(e, index);
                            matches.push(None);
                        }
                    }
                }
                coords.push(floor);
                items.push(item.clone());
                index += 1;
            }
        }
        ItemPlanner {
            coords,
            items,
            matches,
        }
    }

    #[allow(clippy::ptr_arg)]
    fn floor_has_generator(&self, coords: &Coords, floor: usize) -> bool {
        for (item_index, current_floor) in coords.iter().enumerate() {
            if *current_floor == floor {
                if let Item::Generator(_) = self.items[item_index] {
                    if coords[item_index] == floor {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[allow(clippy::ptr_arg)]
    fn coords_valid(&self, coords: &Coords) -> bool {
        for (item_index, current_item) in self.items.iter().enumerate() {
            if let Item::MicroChip(_element) = current_item {
                let current_floor = coords[item_index];
                let other_item_index = self.matches[item_index].unwrap();
                if coords[other_item_index] != current_floor {
                    if self.floor_has_generator(&coords, current_floor) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_elevator_index(&self) -> usize {
        for (item_index, current_item) in self.items.iter().enumerate() {
            if let Item::Elevator = current_item {
                return item_index;
            }
        }
        unreachable!()
    }

    #[allow(clippy::ptr_arg)]
    fn get_valid_progressions_from(&self, coords: &Coords) -> Vec<Coords> {
        let mut progressions = Vec::new();
        let mut floors = Vec::new();

        let elevator = self.get_elevator_index();
        let elevator_floor = coords[elevator];

        if elevator_floor < 3 {
            floors.push(elevator_floor + 1);
        }
        if elevator_floor > 0 {
            floors.push(elevator_floor - 1);
        }

        for floor in floors.iter() {
            let mut candidate = coords.to_owned();
            candidate[elevator] = *floor;

            for i in 0..self.items.len() {
                if i != elevator && coords[i] == elevator_floor {
                    candidate[i] = *floor;

                    if self.coords_valid(&candidate) {
                        progressions.push(candidate.clone());
                    }

                    for j in i + 1..self.items.len() {
                        if j != elevator && coords[j] == elevator_floor {
                            candidate[j] = *floor;
                            if self.coords_valid(&candidate) {
                                progressions.push(candidate.clone());
                            }
                            candidate[j] = elevator_floor;
                        }
                    }

                    candidate[i] = elevator_floor;
                }
            }
        }
        progressions
    }

    fn signature(&self, coords: &Coords) -> (Vec<usize>, Vec<usize>, usize) {
        let mut elevator = 0;
        let mut singles_vec = vec![0, 0, 0, 0];
        let mut pairs_vec = vec![0, 0, 0, 0];

        for (index, &floor) in coords.iter().enumerate() {
            if let Item::Elevator = self.items[index] {
                elevator = floor;
            } else {
                let other = self.matches[index].unwrap();
                if coords[other] == floor {
                    if other < index {
                        pairs_vec[floor] += 1;
                    }
                } else {
                    singles_vec[floor] += 1;
                }
            }
        }

        (singles_vec, pairs_vec, elevator)
    }

    fn deduplicate(&self, candidates: Vec<Coords>) -> Vec<Coords> {
        let mut signatures_found = HashSet::new();
        let mut result = Vec::new();
        for c in candidates.iter() {
            let signature = self.signature(&c);
            if signatures_found.insert(signature) {
                result.push(c.clone());
            }
        }
        result
    }

    fn solve(&self) -> usize {
        let mut visited: HashSet<Coords> = HashSet::new();
        let mut via: HashMap<Coords, Coords> = HashMap::new();
        let mut queue: BinaryHeap<NodeDescriptor> = BinaryHeap::new();
        let distance;

        queue.push(NodeDescriptor {
            coords: self.coords.clone(),
            distance: 0,
        });

        'mainloop: loop {
            let current_node = queue.pop().unwrap();
            let current_coords = current_node.coords.clone();

            let mut done = true;
            for coord in current_coords.iter() {
                if *coord < 3 {
                    done = false;
                    break;
                }
            }
            if done {
                distance = current_node.distance;
                break 'mainloop;
            }

            let candidates = self.get_valid_progressions_from(&current_node.coords);
            let candidates = self.deduplicate(candidates);
            for candidate in candidates.iter() {
                if !visited.contains(candidate) {
                    queue.push(NodeDescriptor {
                        coords: candidate.clone(),
                        distance: current_node.distance + 1,
                    });
                    via.insert(candidate.clone(), current_coords.clone());
                }
            }
            visited.insert(current_coords);
        }

        distance
    }
}

fn parse_element(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_generator(input: &str) -> IResult<&str, Item> {
    let parser = terminated(parse_element, tag(" generator"));
    map(parser, |s| Item::Generator(s))(input)
}

fn parse_microchip(input: &str) -> IResult<&str, Item> {
    let parser = terminated(parse_element, tag("-compatible microchip"));
    map(parser, |s| Item::MicroChip(s))(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    let article_parser = alt((tag("a "), tag("an ")));
    let item_parser = alt((parse_generator, parse_microchip));
    preceded(article_parser, item_parser)(input)
}

fn parse_non_empty_item_list(input: &str) -> IResult<&str, Vec<Item>> {
    let item_parser = preceded(opt(tag("and ")), parse_item);
    separated_list(tag(", "), item_parser)(input)
}

fn parse_empty_item_list(input: &str) -> IResult<&str, Vec<Item>> {
    value(Vec::new(), tag("nothing relevant"))(input)
}

fn parse_item_list(input: &str) -> IResult<&str, Vec<Item>> {
    alt((parse_empty_item_list, parse_non_empty_item_list))(input)
}

fn parse_input_line(input: &str) -> IResult<&str, Vec<Item>> {
    let prelude_parser = tuple((tag("The "), alpha1, tag(" floor contains ")));
    delimited(prelude_parser, parse_item_list, tag("."))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Item>>> {
    separated_list(tag("\n"), parse_input_line)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_rest, mut input) = parse_input(&input).unwrap();
    input[0].push(Item::Elevator);

    let planner = ItemPlanner::new(input.clone());
    println!("Number of steps for part 1: {:?}", planner.solve());

    input[0].push(Item::MicroChip("elerium"));
    input[0].push(Item::Generator("elerium"));
    input[0].push(Item::MicroChip("dilithium"));
    input[0].push(Item::Generator("dilithium"));

    let planner = ItemPlanner::new(input);
    println!("Number of steps for part 2: {:?}", planner.solve());

    Ok(())
}
