use std::io::{self, BufRead};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

/// the output type can be varied - e.g. no Result, single value, whatever you want. Must match up except for fallibility with partN
#[aoc_generator(day1)]
fn generate(input: &[u8]) -> io::Result<Vec<Option<u32>>> {
    input.collect()
}

/// What day, what part
#[aoc(day1, part1)]
fn part1(input: &[Option<u32>]) -> u32 {
    let mut input = input.iter().peekable();
...    largest
}

/// second part
#[aoc(day1, part2)]
fn part2(input: &[Option<u32>]) -> u32 {
    let mut input = input.iter().peekable();
...    sizes.iter().take(3).sum()
}

/// Faster implementation of second part
#[aoc(day1, part2, faster)]
fn part2_fasterinput: &[Option<u32>]) -> u32 {
    let mut input = input.iter().peekable();
... whatever ...
}