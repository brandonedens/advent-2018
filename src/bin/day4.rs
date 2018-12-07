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

/// Read events from the day 4 file.
fn read_events() -> Vec<Event> {
    let file = File::open("input/day4.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut events = reader
        .lines()
        .map(|line| Event::from_line(&line.unwrap()))
        .collect::<Vec<Event>>();
    events.sort();
    events
}

fn compute_guard_sleep(events: &Vec<Event>) -> HashMap<usize, Vec<Sleep>> {
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

    guards
}

fn compute_minute_freq(sleep: &Vec<Sleep>) -> Vec<usize> {
    let one_minute = Duration::minutes(1);
    let mut minutes = vec![0; 60];
    for s in sleep {
        let mut t = s.begin;
        loop {
            minutes[t.time().minute() as usize] += 1;

            t = t + one_minute;
            if t == s.end {
                break;
            }
        }
    }
    minutes
}

fn problem1() {
    let events = read_events();
    let guards = compute_guard_sleep(&events);

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
    let sleepy_guard = guards.get(&sleepy_guard_id).unwrap();
    let minutes = compute_minute_freq(&sleepy_guard);
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

fn problem2() {
    let events = read_events();
    let guards = compute_guard_sleep(&events);
    let guard_minute_freq: Vec<(usize, Vec<usize>)> = guards
        .iter()
        .map(|(k, v)| (*k, compute_minute_freq(v)))
        .collect();
    let mut max_guard_id = None;
    let mut max_minute = None;
    let mut max_sleep = 0;
    for i in 0..60 {
        for (guard_id, minutes) in guard_minute_freq.iter() {
            if minutes[i] > max_sleep {
                max_sleep = minutes[i];
                max_guard_id = Some(guard_id);
                max_minute = Some(i);
            }
        }
    }
    let max_guard_id = max_guard_id.unwrap();
    let max_minute = max_minute.unwrap();
    println!(
        "{} * {} = {}",
        max_guard_id,
        max_minute,
        max_guard_id * max_minute
    );
}

fn main() {
    problem1();
    problem2();
}
