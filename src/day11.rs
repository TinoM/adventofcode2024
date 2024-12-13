use std::collections::HashMap;

use aoc_runner_derive::aoc;


// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. 
//    (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let stones = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    // eprintln!("{:?}", stones);
    let mut p = Pluto{cache:HashMap::new()};
    stones.iter().map(|&s| p.calc(s,25)).sum()
}

struct Pluto {
    cache: HashMap<(usize,usize),usize>
}

impl Pluto {

    fn calc(&mut self,stone:usize,blinks:usize) -> usize {
        if blinks == 0 {
            return 1;
        }
        if stone == 0 {
            return self.calc(1,blinks-1);
        }
        if let Some(precalc) = self.cache.get(&(stone,blinks)) {
            return *precalc;
        }
        let digits = (stone as f64).log10().floor() as u32 + 1;
        if digits % 2 == 0 {
            let left = stone/(10usize.pow(digits/2));
            let right = stone%(10usize.pow(digits/2));
            let r = self.calc(left,blinks-1) + self.calc(right,blinks-1);
            self.cache.insert((stone,blinks),r);
            return r;
        } else {
            let r = self.calc(stone*2024,blinks-1);
            self.cache.insert((stone,blinks),r);
            return r;
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let stones = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    // eprintln!("{:?}", stones);
    let mut p = Pluto{cache:HashMap::new()};
    stones.iter().map(|&s| p.calc(s,75)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}