// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp::{max, min};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod test_point {
    use super::*;

    #[test]
    fn test_distance() {
        let origin = Point::new(0, 0);

        assert_eq!(origin.distance(&Point::new(5, 2)), 7);
        assert_eq!(origin.distance(&Point::new(-5, 2)), 7);
        assert_eq!(origin.distance(&Point::new(-5, -2)), 7);
        assert_eq!(origin.distance(&Point::new(5, -2)), 7);

        let p2 = Point::new(-2, -2);
        assert_eq!(origin.distance(&p2), p2.distance(&origin));
    }
}

const H: i32 = 0;
const V: i32 = 1;

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// Create a new line of start and end points
    fn new(start: Point, end: Point) -> Line {
        Line {
            start: start,
            end: end,
        }
    }

    /// Return direction of this line: H or V
    fn direction(&self) -> i32 {
        if self.start.x == self.end.x {
            V
        } else {
            H
        }
    }

    /// Return whether this and another line intersect
    fn intersect(&self, other: &Self) -> Option<Point> {
        if self.direction() == H && other.direction() == V {
            if min(self.start.x, self.end.x) <= other.start.x
                && max(self.start.x, self.end.x) >= other.start.x
                && min(other.start.y, other.end.y) <= self.start.y
                && max(other.start.y, other.end.y) >= self.start.y
            {
                return Some(Point::new(other.start.x, self.start.y));
            }
        } else if self.direction() == V && other.direction() == H {
            return other.intersect(self);
        }
        None
    }
}

#[cfg(test)]
mod test_line {
    use super::*;

    #[test]
    fn test_direction() {
        // horizontal lines range over x
        assert_eq!(
            Line::new(Point::new(0, 0), Point::new(10, 0)).direction(),
            H
        );
        // vertical lines range over y
        assert_eq!(
            Line::new(Point::new(0, 0), Point::new(0, 10)).direction(),
            V
        );
    }

    #[test]
    fn test_intersect() {
        let line1 = Line::new(Point::new(0, 0), Point::new(10, 0));
        let line2 = Line::new(Point::new(5, -4), Point::new(5, 4));

        // Lines intersect either way
        assert_eq!(line1.intersect(&line2), Some(Point::new(5, 0)));
        assert_eq!(line2.intersect(&line1), Some(Point::new(5, 0)));
    }

    #[test]
    fn test_dont_intersect() {
        let line1 = Line::new(Point::new(0, 0), Point::new(10, 0));
        let line2 = Line::new(Point::new(-1, -4), Point::new(-1, 4));
        //
        // Lines don't intersect either way
        assert_eq!(line1.intersect(&line2), None);
        assert_eq!(line2.intersect(&line1), None);
    }
}

/// Parses a wire into a vector of lines from the origin
fn create_wire(data: &str) -> Vec<Line> {
    let mut start = Point::new(0, 0);
    let mut end: Point;
    let mut lines = Vec::new();

    for item in data.split(",") {
        println!("{:?}: -> {}", start, item);
        let opcode = &item[0..1];
        let num = &item[1..].parse::<i32>().unwrap();

        match &opcode as &str {
            "R" => {
                end = Point::new(start.x + num, start.y);
            }
            "L" => {
                end = Point::new(start.x - num, start.y);
            }
            "U" => {
                end = Point::new(start.x, start.y + num);
            }
            "D" => {
                end = Point::new(start.x, start.y - num);
            }
            _ => {
                eprintln!("Error: opcode {} not valid ({})", opcode, item);
                std::process::exit(exitcode::DATAERR);
            }
        }
        lines.push(Line::new(start, end));
        start = end;
    }

    lines
}

fn main() {
    let datafile_arg = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Error: no data file provided.");
            std::process::exit(exitcode::DATAERR);
        }
    };

    let mut file = match File::open(Path::new(&datafile_arg)) {
        Err(e) => {
            eprintln!("Can't open file: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Can't read file: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    };

    // Parse the wires
    let mut wires = Vec::new();
    let contents = contents.trim();
    for line in contents.split("\n") {
        let line = line.trim();
        println!("WIRE: {}", line);
        // Parse wire and convert it to a series of Lines
        wires.push(create_wire(line));
        println!("parsed: {:?}", wires[wires.len() - 1]);
    }

    let mut intersections = Vec::new();

    // For each line in each wire, check intersections with other wire
    'outer: for wire_lhs_i in 0..wires.len() {
        for wire_rhs_i in 0..wires.len() {
            if wire_lhs_i == wire_rhs_i {
                continue 'outer;
            }
            let wire_lhs = &wires[wire_lhs_i];
            let wire_rhs = &wires[wire_rhs_i];

            for lhs_i in 0..wire_lhs.len() {
                for rhs_i in 0..wire_rhs.len() {
                    let line_lhs = &wire_lhs[lhs_i];
                    let line_rhs = &wire_rhs[rhs_i];

                    match line_lhs.intersect(&line_rhs) {
                        Some(p) => intersections.push(p),
                        None => (),
                    }
                }
            }
        }
    }
    println!("intersections: {:?}", intersections);

    // For each intersection, figure out the distance and then figure out the minimum distance
    let origin = Point::new(0, 0);
    let distances: Vec<i32> = intersections
        .iter()
        .map(|&val| origin.distance(&val))
        .collect();

    println!("distances: {:?}", distances);

    match distances.iter().min() {
        Some(val) => {
            println!("Minimum distance: {}", val);
        }
        None => {
            println!("No minimum distance was found.");
        }
    }
}
