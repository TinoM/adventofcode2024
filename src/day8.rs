use std::collections::{HashMap, HashSet};
use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut antennas:HashMap<char, Vec<(isize,isize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).and_modify(|f|f.push((x as isize, y as isize))).or_insert(vec![(x as isize,y as isize)]);
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, v) in antennas.iter() {
        for a in v.iter() {
            for b in v.iter() {
                if a == b {
                    continue;
                }
                
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                // eprintln!("{} {:?} {:?} => ({},{}) => {:?} {:?}", c, a, b,dx,dy,(a.0-dx,a.1-dy),(b.0+dx,b.1+dy));
                antinodes.insert((a.0+dx,a.1+dy));
                antinodes.insert((b.0-dx,b.1-dy));
            }
        }
    }
    antinodes.retain(|n|n.0 >= 0 && n.1 >= 0 && n.0 < input.lines().next().unwrap().len() as isize && n.1 < input.lines().count() as isize);
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut antennas:HashMap<char, Vec<(isize,isize)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height = height.max(y as isize);
        for (x, c) in line.chars().enumerate() {
            width = width.max(x as isize);          
            if c != '.' {
                antennas.entry(c).and_modify(|f|f.push((x as isize, y as isize))).or_insert(vec![(x as isize,y as isize)]);
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, v) in antennas.iter() {
        for a in v.iter() {
            for b in v.iter() {
                if a == b {
                    continue;
                }
                antinodes.insert(*a);
                antinodes.insert(* b);
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                // eprintln!("{} {:?} {:?} => ({},{}) => {:?} {:?}", c, a, b,dx,dy,(a.0-dx,a.1-dy),(b.0+dx,b.1+dy));
                let mut anti = (a.0+dx,a.1+dy);
                while anti.0 >= 0 && anti.1 >= 0 && anti.0 <= width && anti.1 <= height {
                    antinodes.insert(anti);
                    anti = (anti.0+dx,anti.1+dy);
                }
                anti = (b.0-dx,b.1-dy);
                while anti.0 >= 0 && anti.1 >= 0 && anti.0 <= width && anti.1 <= height {
                    antinodes.insert(anti);
                    anti = (anti.0-dx,anti.1-dy);
                }
            }
        }
    }
    // antinodes.retain(|n|n.0 >= 0 && n.1 >= 0 && n.0 < input.lines().next().unwrap().len() as isize && n.1 < input.lines().count() as isize);
    antinodes.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 34);
    }
}