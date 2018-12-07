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

use std::fs::File;
use std::io::prelude::*;

fn problem1() {
    let file = File::open("input/day5.txt").unwrap();

    let mut v: Vec<char> = Vec::new();
    for c in file.bytes() {
        let c = c.unwrap() as char;

        if !c.is_ascii_alphabetic() {
            continue;
        }

        if v.len() == 0 {
            v.push(c);
            continue;
        }

        let last = v.last().unwrap();
        if c.eq_ignore_ascii_case(last)
            && ((c.is_ascii_uppercase() && last.is_ascii_lowercase())
                || (last.is_ascii_uppercase() && c.is_ascii_lowercase()))
        {
            // We have aA or Aa.
            v.pop();
            continue;
        }

        v.push(c);
    }
    println!("len: {}", v.len());
}

fn problem2() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut counts: Vec<(usize, char)> = Vec::new();

    for a in alphabet.chars() {
        let mut v: Vec<char> = Vec::new();

        let file = File::open("input/day5.txt").unwrap();
        for c in file.bytes() {
            let c = c.unwrap() as char;

            if !c.is_ascii_alphabetic() {
                continue;
            }

            if c.eq_ignore_ascii_case(&a) {
                continue;
            }

            if v.len() == 0 {
                v.push(c);
                continue;
            }

            let last = v.last().unwrap();
            if c.eq_ignore_ascii_case(last)
                && ((c.is_ascii_uppercase() && last.is_ascii_lowercase())
                    || (last.is_ascii_uppercase() && c.is_ascii_lowercase()))
            {
                // We have aA or Aa.
                v.pop();
                continue;
            }

            v.push(c);
        }
        counts.push((v.len(), a));
    }
    counts.sort();

    println!("{:?}", counts.first().unwrap());
}

fn main() {
    problem1();
    problem2();
}
