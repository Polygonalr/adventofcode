use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn p1(line_vec: &[String]) -> i32 {
    let mut res = 0;
    for line in line_vec {
        let mut count = 0;
        let numbers = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let winning_nos = &numbers[1..11];
        let mut winning_map = [false; 100];
        let own_nos = &numbers[11..];
        for winning in winning_nos {
            winning_map[*winning] = true;
        }
        for own in own_nos {
            if winning_map[*own] {
                count += 1;
            }
        }
        if count != 0 {
            let base: i32 = 2;
            res += base.pow(count - 1);
        }
    }
    res
}

fn p2(line_vec: &[String]) -> i32 {
    let mut scratchcard_count = [1; 203];
    scratchcard_count[0] = 0;

    let mut matching_count = [0; 203];

    for line in line_vec {
        let mut count = 0;
        let numbers = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let winning_nos = &numbers[1..11];
        let mut winning_map = [false; 100];
        let own_nos = &numbers[11..];
        for winning in winning_nos {
            winning_map[*winning] = true;
        }
        for own in own_nos {
            if winning_map[*own] {
                count += 1;
            }
        }
        matching_count[numbers[0]] = count;
    }

    for i in 1..line_vec.len() + 1 {
        for j in 1..matching_count[i] + 1 {
            scratchcard_count[i + j] += scratchcard_count[i];
        }
    }
    scratchcard_count.iter().sum()
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
        // line_vec.push(line.to_string());

        // Use the following if only need to process digits
        let to_push = line
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
            .collect::<String>();
        line_vec.push(to_push);
    }
    println!("Part 1: {}\nPart 2: {}", p1(&line_vec), p2(&line_vec));
}
