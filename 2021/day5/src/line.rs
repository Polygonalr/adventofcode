use std::{fmt, mem};
use std::collections::HashSet;
use std::cmp::{min, max};

#[derive(Debug, Clone)]
pub struct Line {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32
}

impl Line {
    pub fn new(mut start_x: i32, mut end_x: i32, mut start_y: i32, mut end_y: i32) -> Line {
        // For simplicity sake, make every line start from left and end at right.
        // If it is a vertical line, make it start from down and end at up.
        // therefore, start_x < end_x, and if start_x = end_x, then start_y < end_y 
        if start_x > end_x || (start_x == end_x && start_y > end_y) {
            mem::swap(&mut start_x, &mut end_x);
            mem::swap(&mut start_y, &mut end_y);
        }
        Line { start_x, end_x, start_y, end_y }
    }

    // check whether 2 lines overlap
    pub fn is_overlap(&self, other: &Line) -> bool {
        let x_overlap = (self.start_x <= other.start_x && self.end_x >= other.start_x) ||
                        (self.start_x <= other.end_x && self.end_x >= other.end_x) ||
                        (other.start_x <= self.start_x && other.end_x >= self.start_x) ||
                        (other.start_x <= self.end_x && other.end_x >= self.end_x);

        let y_overlap = (self.start_y <= other.start_y && self.end_y >= other.start_y) ||
                        (self.start_y <= other.end_y && self.end_y >= other.end_y) ||
                        (other.start_y <= self.start_y && other.end_y >= self.start_y) ||
                        (other.start_y <= self.end_y && other.end_y >= self.end_y);

        x_overlap && y_overlap
    }

    // Find and add intersection points of 2 lines to intersection_set and return
    pub fn add_intersections(&self, other: &Line, mut intersection_set: HashSet<(i32,i32)>) -> Result<HashSet<(i32, i32)>, &'static str> {
        if !self.is_overlap(other) {
            return Err("No intersection");
        }
        if !self.is_diagonal() && !other.is_diagonal() {
            // non-diagonal lines

            // Check whether they are parallel first
            let parallel_x = self.start_x == self.end_x && other.start_x == other.end_x;
            let parallel_y = self.start_y == self.end_y && other.start_y == other.end_y;
            if parallel_x {
                // println!("Parallel (x) lines found: {} and {}", self, other);
                let (start_y, end_y) = (max(self.start_y, other.start_y), min(self.end_y, other.end_y));
                let fixed_x = self.start_x;
                for i in start_y..(end_y + 1) {
                    intersection_set.insert((fixed_x, i));
                }
            } else if parallel_y {
                // println!("Parallel (y) lines found: {} and {}", self, other);
                let (start_x, end_x) = (max(self.start_x, other.start_x), min(self.end_x, other.end_x));
                let fixed_y = self.start_y;
                for i in start_x..(end_x + 1) {
                    intersection_set.insert((i, fixed_y));
                }
            } else {
                // insert intersection point of a horizontal and vertical line
                // check whether self is horizontal
                let (intersect_x, intersect_y): (i32, i32);
                if self.start_y == self.end_y {
                    // other must be vertical
                    intersect_x = other.start_x;
                    intersect_y = self.start_y;
                } else {
                    intersect_x = self.start_x;
                    intersect_y = other.start_y;
                }
                intersection_set.insert((intersect_x, intersect_y));
            }

            return Ok(intersection_set);
        }
        Err("Diagonal lines detected, use naive_compile_points instead")
    }

    pub fn is_diagonal(&self) -> bool {
        self.start_x != self.end_x && self.start_y != self.end_y
    }

    pub fn naive_compile_points(&self) -> Vec<(i32, i32)> {
        let mut all_points: Vec<(i32, i32)> = Vec::new();
        if !self.is_diagonal() {
            if self.start_x == self.end_x {
                for i in self.start_y..(self.end_y + 1) {
                    all_points.push((self.start_x, i));
                }
            } else {
                for i in self.start_x..(self.end_x + 1) {
                    all_points.push((i, self.start_y));
                }
            }
        } else {
            let dx = if self.start_x < self.end_x { 1 } else { -1 };
            let dy = if self.start_y < self.end_y { 1 } else { -1 };
            let mut x = self.start_x;
            let mut y = self.start_y;
            loop {
                all_points.push((x, y));
                x += dx;
                y += dy;
                if x == self.end_x + dx {
                    break;
                }
            }
        }
        all_points
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: ({}, {}) to ({}, {})", self.start_x, self.start_y, self.end_x, self.end_y)
    }
}