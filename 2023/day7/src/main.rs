use std::{cmp::Ordering, collections::HashMap, env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

#[derive(Debug)]
struct Hand {
    val: Vec<char>,
    bid: i64,
}

impl Hand {
    fn new(val: String, bid: i64) -> Hand {
        let val = val.chars().collect::<Vec<_>>();
        Hand {
            val, bid
        }
    }

    fn card_strength(c: char) -> i32 {
        let lookup = HashMap::from([('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);
        match c {
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c as i32 - '0' as i32,
            _ => lookup[&c],
        }
    }

    fn hand_strength(&self) -> i32 {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in self.val.iter() {
            *counts.entry(*c).or_insert(0) += 1;
        }
        let mut v: Vec<i32> = vec![];
        for c in counts.values() {
            v.push(*c);
        }
        v.sort_by(|a, b| b.cmp(a));
        if v[0] == 5 {
            return 6;
        } else if v[0] == 4 {
            return 5;
        } else if v.len() >= 2 && v[0] == 3 && v[1] == 2 {
            return 4;
        } else if v[0] == 3 {
            return 3;
        } else if v.len() >= 2 && v[0] == 2 && v[1] == 2 {
            return 2;
        } else if v[0] == 2 {
            return 1;
        }
        0
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.val.cmp(&other.val) == Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_strength = self.hand_strength();
        let other_strength = other.hand_strength();
        if my_strength != other_strength {
            return my_strength.cmp(&other_strength);
        }
        for i in 0..self.val.len() {
            if self.val[i] != other.val[i] {
                return Hand::card_strength(self.val[i]).cmp(&Hand::card_strength(other.val[i]));
            }
        }
        Ordering::Equal
    }
}


/// Same implementation as Hand, but J's card strength is weakest, and every count of J will increase the highest
/// card counter by 1 in hand_strength()
struct Hand2 {
    val: Vec<char>,
    bid: i64,
}

impl Hand2 {
    fn new(val: String, bid: i64) -> Hand2 {
        let val = val.chars().collect::<Vec<_>>();
        Hand2 {
            val, bid
        }
    }

    fn card_strength(c: char) -> i32 {
        let lookup = HashMap::from([('T', 10), ('J', 0), ('Q', 12), ('K', 13), ('A', 14)]);
        match c {
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c as i32 - '0' as i32,
            _ => lookup[&c],
        }
    }

    fn hand_strength(&self) -> i32 {
        let mut counts: HashMap<char, i32> = HashMap::new();
        let mut j_count = 0;
        for c in self.val.iter() {
            if *c == 'J' {
                j_count += 1;
            } else {
                *counts.entry(*c).or_insert(0) += 1;
            }
        }
        let mut v: Vec<i32> = vec![];
        for c in counts.values() {
            v.push(*c);
        }

        v.sort_by(|a, b| b.cmp(a));
        if v.is_empty() {
            v.push(5);
        } else {
            v[0] += j_count;
        }
        
        if v[0] == 5 {
            return 6;
        } else if v[0] == 4 {
            return 5;
        } else if v.len() >= 2 && v[0] == 3 && v[1] == 2 {
            return 4;
        } else if v[0] == 3 {
            return 3;
        } else if v.len() >= 2 && v[0] == 2 && v[1] == 2 {
            return 2;
        } else if v[0] == 2 {
            return 1;
        }
        0
    }
}

impl Eq for Hand2 {}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.val.cmp(&other.val) == Ordering::Equal
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_strength = self.hand_strength();
        let other_strength = other.hand_strength();
        if my_strength != other_strength {
            return my_strength.cmp(&other_strength);
        }
        for i in 0..self.val.len() {
            if self.val[i] != other.val[i] {
                return Hand2::card_strength(self.val[i]).cmp(&Hand2::card_strength(other.val[i]));
            }
        }
        Ordering::Equal
    }
}

fn p1(line_vec: &[String]) -> i64 {
    let mut hands: Vec<Hand> = line_vec.iter().map(|x| {
        let y = x.split(' ').collect::<Vec<_>>();
        Hand::new(y[0].to_string(), y[1].parse::<i64>().unwrap())
    }).collect();
    hands.sort();
    let mut res: i64 = 0;
    for i in 0..hands.len() {
        res += (i+1) as i64 * hands[i].bid;
    }
    res
}

fn p2(line_vec: &[String]) -> i64 {
    let mut hands: Vec<Hand2> = line_vec.iter().map(|x| {
        let y = x.split(' ').collect::<Vec<_>>();
        Hand2::new(y[0].to_string(), y[1].parse::<i64>().unwrap())
    }).collect();
    hands.sort();
    let mut res: i64 = 0;
    for i in 0..hands.len() {
        res += (i+1) as i64 * hands[i].bid;
    }
    res
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
