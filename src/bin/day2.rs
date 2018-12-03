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

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn problem1() {
    let file = File::open("input/day2.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut two_sum = 0;
    let mut three_sum = 0;
    for line in reader.lines() { 
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in line.unwrap().chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for v in map.values() {
            if *v == 2 {
                two_sum += 1;
                break;
            }
        }
        for v in map.values() {
            if *v == 3 {
                three_sum += 1;
                break;
            }
        }
    }
    println!("{} * {} = {}",
             two_sum, three_sum, two_sum * three_sum);
}

fn main() {
    problem1();
}
