use aoc_runner_derive::{aoc, aoc_generator};

/// the output type can be varied - e.g. no Result, single value, whatever you want. Must match up except for fallibility with partN
#[aoc_generator(day4)]
fn generate(input: &str) -> String {
    input.to_owned()
}

#[inline(always)]
fn fully_contains(u: &[usize], v: &[usize]) -> bool {
    u[0] <= v[0] && u[1] >= v[1]
}

fn overlap(u: &[usize], v: &[usize]) -> bool {
    !((u[1] < v[0]) || (u[0] > v[1]))
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|r| {
                    r.split('-')
                        .map(|n| n.parse::<usize>().expect("cannot parse to usize"))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .filter(|l| fully_contains(&l[0], &l[1]) || fully_contains(&l[1], &l[0]))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|r| {
                    r.split('-')
                        .map(|n| n.parse::<usize>().expect("cannot parse to usize"))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .filter(|l| overlap(&l[0], &l[1]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generate("2-4,6-8",)), 0);
        assert_eq!(part2(&generate("2-3,4-5",)), 0);
        assert_eq!(part2(&generate("5-7,7-9",)), 1);
        assert_eq!(part2(&generate("2-8,3-7",)), 1);
        assert_eq!(
            part2(&generate(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            )),
            4
        );
    }
}
