// Copyright 2018 by Brandon Edens.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2018-12-05

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use chrono::prelude::*;
use chrono::Duration;
use regex::Regex;

lazy_static! {
    // Line is: #10 @ 674,274: 25x13
    static ref RE: Regex =
        Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)$").unwrap();

    static ref BEGINS_SHIFT: Regex =
        Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum Behavior {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Event {
    datetime: DateTime<Utc>,
    behavior: Behavior,
}

impl Event {
    fn from_line(line: &str) -> Self {
        let caps = RE.captures(line).unwrap();
        let year = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let month = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let day = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let hour = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let minute = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

        let datetime = Utc.ymd(year, month, day).and_hms(hour, minute, 0);

        let event = caps.get(6).unwrap().as_str();
        let behavior = if BEGINS_SHIFT.is_match(event) {
            let caps = BEGINS_SHIFT.captures(event).unwrap();
            let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Behavior::BeginsShift(id)
        } else if event == "falls asleep" {
            Behavior::FallsAsleep
        } else if event == "wakes up" {
            Behavior::WakesUp
        } else {
            unreachable!();
        };
        Event {
            datetime: datetime,
            behavior: behavior,
        }
    }
}

#[derive(Debug)]
struct Sleep {
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
}

fn problem1() {
    let file = File::open("input/day4.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut events = reader
        .lines()
        .map(|line| Event::from_line(&line.unwrap()))
        .collect::<Vec<Event>>();
    events.sort();

    // Convert timeline events into guard sleep information.
    let mut current_guard = 0;
    let mut guards: HashMap<usize, Vec<Sleep>> = HashMap::new();
    let mut it = events.iter();
    loop {
        if let Some(event) = it.next() {
            match event.behavior {
                Behavior::BeginsShift(id) => {
                    current_guard = id;
                    guards.entry(id).or_insert(Vec::new());
                }
                Behavior::FallsAsleep => {
                    assert!(guards.contains_key(&current_guard));
                    let wake_up_event = it.next().unwrap();
                    let v = guards.get_mut(&current_guard).unwrap();
                    v.push(Sleep {
                        begin: event.datetime,
                        end: wake_up_event.datetime,
                    });
                }
                _ => {
                    unreachable!();
                }
            }
        } else {
            break;
        }
    }

    // Which guard slept the most?
    let mut total_sleep: Vec<(usize, usize)> = guards
        .iter()
        .map(|(k, v)| {
            let sum = v
                .iter()
                .fold(0, |acc, s| acc + (s.end - s.begin).num_minutes() as usize);
            (sum, *k)
        })
        .collect();
    total_sleep.sort();
    let sleepy_guard_id = total_sleep.last().unwrap().1;

    // What minute were they most asleep.
    let one_minute = Duration::minutes(1);
    let sleepy_guard_sleep = guards.get(&sleepy_guard_id).unwrap();
    let mut minutes = vec![0; 60];
    for sleep in sleepy_guard_sleep {
        let mut t = sleep.begin;
        loop {
            minutes[t.time().minute() as usize] += 1;

            t = t + one_minute;
            if t == sleep.end {
                break;
            }
        }
    }
    let mut c: Vec<(usize, usize)> = minutes.iter().enumerate().map(|(e, v)| (*v, e)).collect();
    c.sort();

    let (_, minute) = c.last().unwrap();
    println!(
        "{} * {} = {}",
        sleepy_guard_id,
        minute,
        sleepy_guard_id * minute
    );
}

fn main() {
    problem1();
}
