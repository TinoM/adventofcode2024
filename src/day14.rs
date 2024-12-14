use std::collections::HashMap;

use aoc_runner_derive::aoc;
use regex::Regex;

const WIDTH:i32 = 101;
const HEIGHT:i32 = 103;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {

    let mut robots = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            let vx = cap[3].parse::<i32>().unwrap();
            let vy = cap[4].parse::<i32>().unwrap();
            (x, y, vx, vy)
        })
        .collect::<Vec<(i32, i32, i32, i32)>>();
    for r in robots.iter_mut() {
        if r.2 < 0 {
            r.0 = (r.0 + (100 * (WIDTH + r.2))) % WIDTH;
        } else {
            r.0 = (r.0 + (100 * r.2)) % WIDTH;
        }
        if r.3 < 0 {
            r.1 = (r.1 + (100 * (HEIGHT + r.3))) % HEIGHT;
        } else {
            r.1 = (r.1 + (100 * r.3)) % HEIGHT;
        }
    }
    let mut set = HashMap::new();
    for r in robots.iter() {
        set.entry((r.0, r.1)).and_modify(|c| *c += 1).or_insert(1);
    }
    let q1: i32 = set
        .iter()
        .filter(|r| r.0 .0 < WIDTH / 2 && r.0 .1 < HEIGHT / 2)
        .map(|x| x.1)
        .sum();
    let q2: i32 = set
        .iter()
        .filter(|r| r.0 .0 > WIDTH / 2 && r.0 .1 < HEIGHT / 2)
        .map(|x| x.1)
        .sum();
    let q3: i32 = set
        .iter()
        .filter(|r| r.0 .0 < WIDTH / 2 && r.0 .1 > HEIGHT / 2)
        .map(|x| x.1)
        .sum();
    let q4: i32 = set
        .iter()
        .filter(|r| r.0 .0 > WIDTH / 2 && r.0 .1 > HEIGHT / 2)
        .map(|x| x.1)
        .sum();
    // eprintln!("Q1: {} Q2: {} Q3: {} Q4: {}", q1, q2, q3, q4);
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    const WIDTH:i32 = 101;
    const HEIGHT:i32 = 103;
    let mut robots = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            let vx = cap[3].parse::<i32>().unwrap();
            let vy = cap[4].parse::<i32>().unwrap();
            (x, y, vx, vy)
        })
        .collect::<Vec<(i32, i32, i32, i32)>>();
    'second: for second in 1..=WIDTH * HEIGHT {
        for r in robots.iter_mut() {
            if r.2 < 0 {
                r.0 = (r.0 + (1 * (WIDTH + r.2))) % WIDTH;
            } else {
                r.0 = (r.0 + (1 * r.2)) % WIDTH;
            }
            if r.3 < 0 {
                r.1 = (r.1 + (1 * (HEIGHT + r.3))) % HEIGHT;
            } else {
                r.1 = (r.1 + (1 * r.3)) % HEIGHT;
            }
        }
        let mut set = [false;WIDTH as usize*HEIGHT as usize];
        for r in robots.iter() {
            // set.entry((r.0, r.1)).and_modify(|c| *c += 1).or_insert(1);
            if set[(r.0 + r.1 * WIDTH) as usize] {
                continue 'second;
            } else {
                set[(r.0 + r.1 * WIDTH) as usize] = true;
            }
        }
        return second as i32;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 12);
    }

}
