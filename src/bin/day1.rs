// Copyright 2018 by Brandon Edens.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2018-12-02

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn repeat_freq() -> i32 {
    let file = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(&file);

    let readings: Vec<i32> = reader.lines().map(|x| {
        x.unwrap().parse::<i32>().unwrap()
    }).collect();

    let mut seen: HashSet<i32> = HashSet::new();
    let mut freq = 0;
    let it = readings.iter().cycle();
    for v in it {
        freq += v;
        if seen.contains(&freq) {
            return freq;
        }
        seen.insert(freq);
    }

    0
}

fn main() {
    let file = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(&file);
    let freq: i32 = reader.lines().map(|x| {
        x.unwrap().parse::<i32>().unwrap()
    }).sum();
    println!("freq: {}", freq);

    print!("rep freq: {}", repeat_freq());
}
