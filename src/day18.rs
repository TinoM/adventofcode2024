use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let tmp = l.split(',').flat_map(|p| p.parse()).collect::<Vec<usize>>();
            tmp[0] + tmp[1] * SIZE
        })
        .collect()
}

const SIZE: usize = 71;
const BYTES: usize = 1024;

#[aoc(day18, part1)]
fn part1(corrupted: &Vec<usize>) -> usize {
    // eprintln!("{:?}",corrupted);
    let mut frontier = VecDeque::new();
    frontier.push_front((0, 0));
    let mut visited = HashSet::new();
    visited.insert(0);
    while let Some(current) = frontier.pop_front() {
        if current.0 == (SIZE - 1) + (SIZE - 1) * SIZE {
            // for y in 0..SIZE {
            //     for x in 0..SIZE {
            //         if corrupted[..BYTES].contains(&(x+y*SIZE)) {
            //             eprint!("#");
            //         } else if visited.contains(&(x+y*SIZE)) {
            //             eprint!("O");
            //         } else {
            //             eprint!(".");
            //         }
            //     }
            //     eprintln!();
            // }
            return current.1;
        }
        let x = current.0 % SIZE;
        let y = current.0 / SIZE;
        // eprintln!("Current {},{} after {} steps",x,y,current.1);
        // eprintln!("{:?}",frontier);
        if x > 0 {
            if !visited.contains(&(current.0 - 1)) && !corrupted[..BYTES].contains(&(current.0 - 1))
            {
                frontier.push_back((current.0 - 1, current.1 + 1));
                visited.insert(current.0 - 1);
            }
        }
        if x < SIZE - 1 {
            if !visited.contains(&(current.0 + 1)) && !corrupted[..BYTES].contains(&(current.0 + 1))
            {
                frontier.push_back((current.0 + 1, current.1 + 1));
                visited.insert(current.0 + 1);
            }
        }
        if y > 0 {
            if !visited.contains(&(current.0 - SIZE))
                && !corrupted[..BYTES].contains(&(current.0 - SIZE))
            {
                frontier.push_back((current.0 - SIZE, current.1 + 1));
                visited.insert(current.0 - SIZE);
            }
        }
        if y < SIZE - 1 {
            if !visited.contains(&(current.0 + SIZE))
                && !corrupted[..BYTES].contains(&(current.0 + SIZE))
            {
                frontier.push_back((current.0 + SIZE, current.1 + 1));
                visited.insert(current.0 + SIZE);
            }
        }
    }

    0
}

#[aoc(day18, part2)]
fn part2(corrupted: &Vec<usize>) -> String {
    'outer: for byte in 1..corrupted.len() {
        // eprintln!("Byte corrupted: {},{}", corrupted[byte] % SIZE, corrupted[byte] / SIZE);
        let mut frontier = VecDeque::new();
        frontier.push_front((0, 0));
        let mut visited = HashSet::new();
        visited.insert(0);
        while let Some(current) = frontier.pop_front() {
            if current.0 == (SIZE - 1) + (SIZE - 1) * SIZE {
                // eprintln!(
                //     "Reached end despise corrupted bytes {:?}",
                //     &corrupted[..byte]
                // );
                // for y in 0..SIZE {
                //     for x in 0..SIZE {
                //         if corrupted[..byte].contains(&(x + y * SIZE)) {
                //             eprint!("#");
                //         } else if visited.contains(&(x + y * SIZE)) {
                //             eprint!("O");
                //         } else {
                //             eprint!(".");
                //         }
                //     }
                //     eprintln!();
                // }
                continue 'outer;
            }
            let x = current.0 % SIZE;
            let y = current.0 / SIZE;
            // eprintln!("Current {},{} after {} steps",x,y,current.1);
            // eprintln!("{:?}",frontier);
            if x > 0 {
                if !visited.contains(&(current.0 - 1))
                    && !corrupted[..byte].contains(&(current.0 - 1))
                {
                    frontier.push_back((current.0 - 1, current.1 + 1));
                    visited.insert(current.0 - 1);
                }
            }
            if x < SIZE - 1 {
                if !visited.contains(&(current.0 + 1))
                    && !corrupted[..byte].contains(&(current.0 + 1))
                {
                    frontier.push_back((current.0 + 1, current.1 + 1));
                    visited.insert(current.0 + 1);
                }
            }
            if y > 0 {
                if !visited.contains(&(current.0 - SIZE))
                    && !corrupted[..byte].contains(&(current.0 - SIZE))
                {
                    frontier.push_back((current.0 - SIZE, current.1 + 1));
                    visited.insert(current.0 - SIZE);
                }
            }
            if y < SIZE - 1 {
                if !visited.contains(&(current.0 + SIZE))
                    && !corrupted[..byte].contains(&(current.0 + SIZE))
                {
                    frontier.push_back((current.0 + SIZE, current.1 + 1));
                    visited.insert(current.0 + SIZE);
                }
            }
        }
        for y in 0..SIZE {
            for x in 0..SIZE {
                if corrupted[..byte].contains(&(x + y * SIZE)) {
                    eprint!("#");
                } else if visited.contains(&(x + y * SIZE)) {
                    eprint!("O");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        return format!("{},{}", corrupted[byte-1] % SIZE, corrupted[byte-1] / SIZE);
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "6,1");
    }
}
