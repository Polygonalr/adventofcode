use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn store_all_coordinates(input:Vec<String>) -> Vec<(i32, i32, i32)>{
    let mut current_coordinates = (0, 0, 0); // x, y, length
    let mut coordinate_store = Vec::new();
    for path in input {
        let direction_string = &path[..1];
        let direction = match direction_string {
            "L" => (1, 0),
            "R" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0)
        };
        let unit_string = &path[1..];
        let unit:usize = unit_string.parse().unwrap();
        drop(unit_string);
        
        for _ in 0..unit{
            // Move 1 unit and store coordinates, rinse and repeat
            current_coordinates.0 += direction.0;
            current_coordinates.1 += direction.1;
            current_coordinates.2 += 1;
            let mut to_push = true;
            // Check whether coordinates are already stored
            if coordinate_store.iter().any(|&i:&(i32,i32,i32)| i.0==current_coordinates.0 && i.1==current_coordinates.1) {
                to_push = false;
            }
            if to_push {
                coordinate_store.push(current_coordinates);
            }
        }
    }
    coordinate_store
}

fn main() {
    println!("RUNNING! This will take awhile.");
    let filepath = "./input.txt";
    let (mut vec_a, mut vec_b) = (Vec::new(), Vec::new());

    // Reading and storing coordinates
    if let Ok(lines) = read_lines(filepath){
        let mut line_number = 0;
        for line in lines {
            if let Ok(s) = line{
                let vec_input:Vec<String> = s.split(",").map(|s| s.to_string()).collect();
                if line_number == 0 {
                    vec_a = store_all_coordinates(vec_input);
                    println!("Size of vec_a: {}", vec_a.len());
                } else if line_number == 1{
                    vec_b = store_all_coordinates(vec_input);
                    println!("Size of vec_b: {}", vec_b.len());
                }
            }
            line_number += 1;
        }
    }

    // Processing of data
    #[allow(unused_assignments)]
    let mut matching_a = Vec::new();
    #[allow(unused_assignments)]
    let mut matching_b = Vec::new();

    // Filter both vectors to leave entries with the matching coordinates (i.e. matching x.0 and x.1)
    matching_a = vec_a.iter().filter(|k| vec_b.iter().any(|l| k.0 == l.0 && k.1 == l.1)).collect();
    matching_b = vec_b.iter().filter(|k| matching_a.iter().any(|l| k.0 == l.0 && k.1 == l.1)).collect();
    // Sort both resultant vectors to ensure that matching_a[x] contains the same coordinates as matching_b[x]
    matching_a.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    matching_b.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Iterate through the vectors to calculate distances and lengths
    println!("Listing all matching coordinates between the two vecs...");
    let mut intersection_distances = Vec::new();
    let mut intersection_wire_lengths = Vec::new();
    for (i,x) in matching_a.iter().enumerate() {
        println!("{}, {}", (x.0), (x.1));
        intersection_distances.push(x.0.abs() + x.1.abs());
        intersection_wire_lengths.push(x.2+matching_b[i].2)
    }

    // Iterate through both intersection vectors to find minimum value
    let mut min_value = intersection_distances[0];
    for x in intersection_distances {
        if x <= min_value {
            min_value = x;
        }
    }
    println!("Part 1 answer: {}", min_value);

    min_value = intersection_wire_lengths[0];
    for x in intersection_wire_lengths {
        if x <= min_value {
            min_value = x;
        }
    }
    println!("Part 2 answer: {}", min_value);

    println!("DONE RUNNING!");
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}