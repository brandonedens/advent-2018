// Copyright 2018 by Brandon Edens.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2018-12-03

#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

lazy_static! {
    // Line is: #10 @ 674,274: 25x13
    static ref RE: Regex =
        Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x: x, y: y }
    }
}

struct Bounds {
    top_left: Coord,
    bottom_right: Coord,
}

impl Bounds {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Bounds {
            top_left: Coord { x: x1, y: y1 },
            bottom_right: Coord { x: x2, y: y2 },
        }
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.top_left.x
            && x <= self.bottom_right.x
            && y >= self.top_left.y
            && y <= self.bottom_right.y
    }
}

impl Claim {
    fn from_line(line: &str) -> Self {
        let caps = RE.captures(line).unwrap();
        Claim { id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        x: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        y: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        width: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        height: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let self_bounds = Bounds::new(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
        );
        let other_bounds = Bounds::new(
            other.x,
            other.y,
            other.x + other.width - 1,
            other.y + other.height - 1,
        );
        self_bounds.contains(other.x, other.y)
            || self_bounds.contains(other.x, other.y + other.height - 1)
            || self_bounds.contains(other.x + other.width - 1, other.y)
            || self_bounds.contains(other.x + other.width - 1, other.y + other.height - 1)
            || other_bounds.contains(self.x, self.y)
            || other_bounds.contains(self.x, self.y + self.height - 1)
            || other_bounds.contains(self.x + self.width - 1, self.y)
            || other_bounds.contains(self.x + self.width - 1, self.y + self.height - 1)
    }
}

fn problem1() {
    let file = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(&file);
    let claims = reader.lines().map(|line| Claim::from_line(&line.unwrap())).collect::<Vec<Claim>>();

    let mut grid = Box::new([0usize; 1000 * 1000]);
    for claim in claims {
        for i in 0..claim.height {
            for j in 0..claim.width {
                let y = i + claim.y;
                let x = j + claim.x;
                grid[y * 1000 + x] += 1;
            }
        }
    }

    let count: usize = grid.iter().filter(|x| **x > 1).count();
    println!("overlap: {}", count);
}

fn problem2() {
    let file = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(&file);
    let claims = reader
        .lines()
        .map(|line| Claim::from_line(&line.unwrap()))
        .collect::<Vec<Claim>>();

    let mut overlaps = vec![false; claims.len()];
    for i in 0..claims.len() {
        if overlaps[i] {
            continue;
        }

        for j in 0..claims.len() {
            if i == j {
                continue;
            }

            if claims[i].overlaps(&claims[j]) {
                overlaps[i] = true;
                overlaps[j] = true;
                break;
            }
        }
    }

    if let Some((i, _)) = overlaps.iter().enumerate().find(|(i, b)| !*b) {
        println!("claim: {:#?}", claims[i]);
    }
}

fn main() {
    problem1();
    problem2();
}

mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let c1 = Claim {
            id: 1,
            x: 3,
            y: 4,
            width: 5,
            height: 6,
        };
        let c2 = Claim {
            id: 2,
            x: 10,
            y: 4,
            width: 5,
            height: 6,
        };
        assert!(!c1.overlaps(&c2));
        let c2 = Claim {
            id: 2,
            x: 7,
            y: 4,
            width: 5,
            height: 6,
        };
        assert!(c1.overlaps(&c2));
    }

    #[test]
    fn test_overlap2() {
        let c1 = Claim {
            id: 1395,
            x: 478,
            y: 475,
            width: 9,
            height: 9,
        };
        let c2 = Claim {
            id: 188,
            x: 907,
            y: 475,
            width: 29,
            height: 14,
        };
        assert!(!c1.overlaps(&c2));
        assert!(!c2.overlaps(&c1));
    }
}
