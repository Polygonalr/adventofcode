use std::collections::{HashMap, HashSet, VecDeque};
use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";
const ROUNDS_TO_SIMULATE: usize = 10;

/**
 * x increases from left to right
 * y increases from top to bottom
 */
#[derive(Eq, Hash, PartialEq, Debug, Default)]
struct Elf {
    x: i64,
    y: i64,
    next_move: Option<(i64, i64)>,
    moved_previous_round: bool,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Elf {
    fn compute_next_move(&mut self, grid: &HashSet<(i64, i64)>, order: &VecDeque<Direction>) {
        // check all 8 directions
        let mut will_move = false;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let to_check = (self.x + dx, self.y + dy);
                if grid.contains(&to_check) {
                    will_move = true;
                    break;
                }
            }
        }
        if !will_move {
            self.next_move = Some((self.x, self.y));
            return;
        }

        let mut dir_queue = VecDeque::from_iter(order.iter().clone());
        while let Some(dir) = dir_queue.pop_front() {
            let (dx, dy): (Vec<i64>, Vec<i64>) = match dir {
                Direction::North => (vec![-1, 0, 1], vec![-1]),
                Direction::South => (vec![-1, 0, 1], vec![1]),
                Direction::West => (vec![-1], vec![-1, 0, 1]),
                Direction::East => (vec![1], vec![-1, 0, 1]),
            };

            let mut to_move = true;
            'x: for x in &dx {
                for y in &dy {
                    let to_check = (self.x + x, self.y + y);
                    if grid.contains(&to_check) {
                        to_move = false;
                        break 'x;
                    }
                }
            }
            if to_move {
                self.next_move = match dir {
                    Direction::North => Some((self.x, self.y - 1)),
                    Direction::South => Some((self.x, self.y + 1)),
                    Direction::West => Some((self.x - 1, self.y)),
                    Direction::East => Some((self.x + 1, self.y)),
                };
                break;
            }
        }

        if self.next_move.is_none() {
            // all directions are occupied, the elf remains where he is
            self.next_move = Some((self.x, self.y));
        }
    }

    fn make_next_move(&mut self, next_move_count: &HashMap<(i64, i64), usize>) {
        assert!(self.next_move.is_some());
        self.moved_previous_round = false;
        let next_move = self.next_move.unwrap();
        // move if no other elf is moving to the same tile
        if next_move_count[&next_move] == 1 {
            if self.x != next_move.0 || self.y != next_move.1 {
                self.moved_previous_round = true;
            }
            self.x = next_move.0;
            self.y = next_move.1;
        }
        self.next_move = None;
    }
}

fn p1(line_vec: &[String]) -> i64 {
    // initialisation
    let mut elves: Vec<Elf> = vec![];
    for (y, line) in line_vec.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Elf {
                    x: x as i64,
                    y: y as i64,
                    ..Default::default()
                });
            }
        }
    }

    // compute
    let mut dir_order = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for _ in 0..ROUNDS_TO_SIMULATE {
        let elf_coords = elves.iter().map(|e| (e.x, e.y)).clone();
        let grid = HashSet::from_iter(elf_coords);

        elves
            .iter_mut()
            .for_each(|elf| elf.compute_next_move(&grid, &dir_order));
        let next_moves_vec: Vec<_> = elves.iter().map(|elf| elf.next_move.unwrap()).collect();
        let mut next_moves_count: HashMap<(i64, i64), usize> = HashMap::new();
        next_moves_vec.iter().for_each(|(x, y)| {
            next_moves_count
                .entry((*x, *y))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });

        elves
            .iter_mut()
            .for_each(|elf| elf.make_next_move(&next_moves_count));
        if let Some(front) = dir_order.pop_front() {
            dir_order.push_back(front);
        } else {
            panic!();
        }
    }

    // compute answer
    let smallest_x = elves.iter().map(|elf| elf.x).min().unwrap();
    let largest_x = elves.iter().map(|elf| elf.x).max().unwrap();
    let smallest_y = elves.iter().map(|elf| elf.y).min().unwrap();
    let largest_y = elves.iter().map(|elf| elf.y).max().unwrap();

    let width = largest_x - smallest_x + 1;
    let height = largest_y - smallest_y + 1;
    (width * height) - elves.len() as i64
}

fn p2(line_vec: &[String]) -> i32 {
    // initialisation
    let mut elves: Vec<Elf> = vec![];
    for (y, line) in line_vec.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Elf {
                    x: x as i64,
                    y: y as i64,
                    ..Default::default()
                });
            }
        }
    }

    // compute
    let mut dir_order = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    let mut round_no = 0;
    loop {
        round_no += 1;
        let elf_coords = elves.iter().map(|e| (e.x, e.y)).clone();
        let grid = HashSet::from_iter(elf_coords);

        elves
            .iter_mut()
            .for_each(|elf| elf.compute_next_move(&grid, &dir_order));
        let next_moves_vec: Vec<_> = elves.iter().map(|elf| elf.next_move.unwrap()).collect();
        let mut next_moves_count: HashMap<(i64, i64), usize> = HashMap::new();
        next_moves_vec.iter().for_each(|(x, y)| {
            next_moves_count
                .entry((*x, *y))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });
        elves
            .iter_mut()
            .for_each(|elf| elf.make_next_move(&next_moves_count));
        let t: i32 = elves
            .iter()
            .map(|elf| elf.moved_previous_round as i32)
            .sum();
        if t == 0 {
            return round_no;
        }

        if let Some(front) = dir_order.pop_front() {
            dir_order.push_back(front);
        } else {
            panic!();
        }
    }
}

fn main() {
    let filepath = if env::args().any(|x| x == *TEST_FLAG) {
        TEST_FILEPATH
    } else {
        INPUT_FILEPATH
    };
    let input = fs::read_to_string(filepath).unwrap();
    let mut line_vec: Vec<String> = Vec::new();
    for line in input.lines() {
        // Process each line...
        line_vec.push(line.to_string());

        // Use the following if only need to process digits
        // let to_push = line
        //     .chars()
        //     .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
        //     .collect::<String>();
        // line_vec.push(to_push);
    }
    println!("Part 1: {}\nPart 2: {}", p1(&line_vec), p2(&line_vec));
}
