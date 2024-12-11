use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().flat_map(|c|c.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut starts = vec![];
    for (y,line) in grid.iter().enumerate() {
        for (x,c) in line.iter().enumerate() {
            if c == &0 {
                // trailheads.insert((x,y), 0);
                starts.push((x,y)); 
            }
        }
    }
    // eprintln!("Trailheads {:?}",starts.len());
    let mut solution = 0;
    for start in starts {
        let mut frontier = vec![(start.0,start.1)];
        let mut solutions = HashSet::new();
        while let Some(current) = frontier.pop() {
            // eprintln!("Current {:?}",current);
            if grid[current.1][current.0] == 9 {
                solutions.insert(current);
                continue;
            }
            if current.0 > 0 && grid[current.1][current.0-1] == grid[current.1][current.0]+1 {
                frontier.push((current.0-1,current.1));
            }
            if current.0 < grid[0].len()-1 && grid[current.1][current.0+1] == grid[current.1][current.0]+1 {
                frontier.push((current.0+1,current.1));
            }
            if current.1 > 0 && grid[current.1-1][current.0] == grid[current.1][current.0]+1 {
                frontier.push((current.0,current.1-1));
            }
            if current.1 < grid.len()-1 && grid[current.1+1][current.0] == grid[current.1][current.0]+1 {
                frontier.push((current.0,current.1+1));
            }
        }
        solution += solutions.len();
    }
    solution
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().flat_map(|c|c.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut trailheads:HashMap<(usize,usize),usize> = HashMap::new();
    let mut frontier = vec![];
    for (y,line) in grid.iter().enumerate() {
        for (x,c) in line.iter().enumerate() {
            if c == &0 {
                // trailheads.insert((x,y), 0);
                frontier.push((x,y,(x,y))); 
            }
        }
    }
    while let Some(current) = frontier.pop() {
        // eprintln!("Current {:?}",current);
        if grid[current.1][current.0] == 9 {
            trailheads.entry(current.2).and_modify(|e| *e+=1).or_insert({
                1
            });
            continue;
        }
        if current.0 > 0 && grid[current.1][current.0-1] == grid[current.1][current.0]+1 {
            frontier.push((current.0-1,current.1,current.2));
        }
        if current.0 < grid[0].len()-1 && grid[current.1][current.0+1] == grid[current.1][current.0]+1 {
            frontier.push((current.0+1,current.1,current.2));
        }
        if current.1 > 0 && grid[current.1-1][current.0] == grid[current.1][current.0]+1 {
            frontier.push((current.0,current.1-1,current.2));
        }
        if current.1 < grid.len()-1 && grid[current.1+1][current.0] == grid[current.1][current.0]+1 {
            frontier.push((current.0,current.1+1,current.2));
        }
    }
    trailheads.iter().map(|th|th.1).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 81);
    }
}