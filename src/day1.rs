use std::io::{self, BufRead};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

/// the output type can be varied - e.g. no Result, single value, whatever you want. Must match up except for fallibility with partN
#[aoc_generator(day1)]
fn generate(input: &[u8]) -> io::Result<Vec<Option<u32>>> {
    input
        .lines()
        .map_ok(|l| -> Result<Option<u32>, std::num::ParseIntError> {
            Ok(if !l.is_empty() {
                Some(l.parse::<u32>()?)
            } else {
                None
            })
        })
        .flatten_ok()
        .collect()
}

/// What day, what part
#[aoc(day1, part1)]
fn part1(input: &[Option<u32>]) -> u32 {
    let mut input = input.iter().peekable();
    let mut largest = 0;
    while input.peek().is_some() {
        largest = std::cmp::max(
            input.by_ref().take_while(|i| i.is_some()).flatten().sum(),
            largest,
        );
    }
    largest
}

/// second part
#[aoc(day1, part2)]
fn part2(input: &[Option<u32>]) -> u32 {
    let mut input = input.iter().peekable();
    let mut totals: Vec<u32> = Vec::new();
    while input.peek().is_some() {
        totals.push(input.by_ref().take_while(|i| i.is_some()).flatten().sum());
    }
    totals.sort();
    totals.sort_by_key(|w| std::cmp::Reverse(*w));
    totals.iter().take(3).sum()
}

// /// Faster implementation of second part
// #[aoc(day1, part2, faster)]
// fn part2_fasterinput: &[Option<u32>]) -> u32 {
// //     let mut input = input.iter().peekable();
// // ... whatever ...
// }
