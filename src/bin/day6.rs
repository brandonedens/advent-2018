// Copyright 2018 by Brandon Edens.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2018-12-06

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use ansi_term::Colour::{Blue, Red};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Line is: 181, 184
    static ref RE: Regex =
        Regex::new(r"^(\d+), (\d+)$").unwrap();
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from_line(line: &str) -> Self {
        let caps = RE.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        Coord { x: x, y: y }
    }

    fn manhatten_distance(&self, other: &Coord) -> usize {
        let x1 = self.x as isize;
        let y1 = self.y as isize;
        let x2 = other.x as isize;
        let y2 = other.y as isize;
        (isize::abs(x1 - x2) + isize::abs(y1 - y2)) as usize
    }
}

/// Convert input data to coordinates.
fn file_to_coords() -> Vec<Coord> {
    let file = File::open("input/day6.txt").unwrap();
    let reader = BufReader::new(&file);
    reader
        .lines()
        .map(|line| Coord::from_line(&line.unwrap()))
        .collect::<Vec<Coord>>()
}

#[derive(Debug)]
struct Bounds {
    top_left: Coord,
    bottom_right: Coord,
}

impl Bounds {
    /// Compute the maximum grid size required to represent all coordinates.
    fn from_coords(coords: &[Coord]) -> Self {
        let min_x = coords.iter().map(|c| c.x).min().unwrap();
        let max_x = coords.iter().map(|c| c.x).max().unwrap();
        let min_y = coords.iter().map(|c| c.y).min().unwrap();
        let max_y = coords.iter().map(|c| c.y).max().unwrap();

        Bounds {
            top_left: Coord { x: min_x, y: min_y },
            bottom_right: Coord { x: max_x, y: max_y },
        }
    }

    /// Return the width of the bounds.
    fn width(&self) -> usize {
        self.bottom_right.x - self.top_left.x + 1
    }

    /// Return the height of the bounds.
    fn height(&self) -> usize {
        self.bottom_right.y - self.top_left.y + 1
    }

    fn normalize(&self, coord: &Coord) -> Coord {
        Coord {
            x: coord.x - self.top_left.x,
            y: coord.y - self.top_left.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Equal,
    Occupied(usize),
    Closest(usize),
}

fn usize_to_char(v: &usize) -> char {
    let v = *v as u8;
    if v < 26 {
        (v + ('A' as u8)) as char
    } else {
        ((v - 26) + ('a' as u8)) as char
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Equal => write!(f, "."),
            Cell::Occupied(x) => write!(f, "{}", Red.paint(format!("{}", usize_to_char(x)))),
            Cell::Closest(x) => write!(f, "{}", Blue.paint(format!("{}", usize_to_char(x)))),
        }
    }
}

fn problem1() {
    let coords = file_to_coords();
    let bounds = Bounds::from_coords(&coords);

    let mut grid = vec![Cell::Empty; bounds.width() * bounds.height()];
    // Walk through the coordinates assigning them to the grid.
    coords.iter().enumerate().for_each(|(e, coord)| {
        let normalized = bounds.normalize(coord);
        grid[normalized.y * bounds.width() + normalized.x] = Cell::Occupied(e);
    });

    // Walk through the grid and for each empty cell determine which coordinate is closest or we
    // have equal distance.
    for i in 0..bounds.height() {
        for j in 0..bounds.width() {
            if grid[i * bounds.width() + j] != Cell::Empty {
                continue;
            }

            let cur = Coord { x: j, y: i };
            let mut min_dist = bounds.height() + bounds.width();

            for (e, coord) in coords.iter().enumerate() {
                let dist = cur.manhatten_distance(&bounds.normalize(coord));
                if dist < min_dist {
                    grid[i * bounds.width() + j] = Cell::Closest(e);
                    min_dist = dist;
                } else if dist == min_dist {
                    grid[i * bounds.width() + j] = Cell::Equal;
                }
            }
        }
    }

    // Remove set of coordinates that extend to infinity.
    let mut infinity_set: HashSet<usize> = HashSet::new();
    for i in 0..bounds.width() {
        match grid[i] {
            Cell::Occupied(x) | Cell::Closest(x) => {
                infinity_set.insert(x);
            }
            _ => {}
        }
        match grid[(bounds.height() - 1) * bounds.width() + i] {
            Cell::Occupied(x) | Cell::Closest(x) => {
                infinity_set.insert(x);
            }
            _ => {}
        }
    }
    for i in 0..bounds.height() {
        match grid[i * bounds.width()] {
            Cell::Occupied(x) | Cell::Closest(x) => {
                infinity_set.insert(x);
            }
            _ => {}
        }
        match grid[(i * bounds.width()) + bounds.width() - 1] {
            Cell::Occupied(x) | Cell::Closest(x) => {
                infinity_set.insert(x);
            }
            _ => {}
        }
    }

    // Compute the sections with largest area.
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..bounds.height() {
        for j in 0..bounds.width() {
            let v = &grid[i * bounds.width() + j];
            match v {
                Cell::Occupied(id) | Cell::Closest(id) => {
                    if infinity_set.contains(&id) {
                        continue;
                    }
                    *map.entry(*id).or_insert(0) += 1;
                }
                _ => {}
            }
        }
    }

    let mut vec = map
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<Vec<(usize, usize)>>();
    vec.sort();
    let largest = vec.last().unwrap();
    println!("largest: {:#?}", largest);

    /*
    for i in 0..bounds.height() {
        for j in 0..bounds.width() {
            print!("{}", grid[i * bounds.width() + j])
        }
        println!("");
    }
    */
}

fn main() {
    problem1();
}
