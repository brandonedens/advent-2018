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

fn main() {
    problem1();
}
