use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1,old)]
pub fn part1_old(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a: usize = cap[1].parse().unwrap();
            let b: usize = cap[2].parse().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part1, speed)]
pub fn part1(input: &str) -> usize {
    let mut chars = input.chars();
    let mut sum = 0;
    let mut counter = 0;
    let mut a = 0;
    let mut b = 0;
    while let Some(c) = chars.next() {
        match c {
            'm' => {
                if counter == 0 {
                    counter += 1;
                } else {
                    counter = 1;
                    a = 0;
                    b = 0;
                }
            }
            'u' => {
                if counter == 1 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
            }
            'l' => {
                if counter == 2 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
            }
            '(' => {
                if counter == 3 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
            }
            ',' => {
                if counter == 4 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
            }
            ')' => {
                if counter == 5 {
                    sum += a * b;
                }
                counter = 0;
                a = 0;
                b = 0;
            },
            c => match c.to_digit(10) {
                Some(d) => {
                    if counter == 4 {
                        a = a * 10 + d as usize;
                    } else if counter == 5 {
                        b = b * 10 + d as usize;
                    }
                }
                None => {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
            },
        }
    }
    sum
}

#[aoc(day3, part2,old)]
pub fn part2_old(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.captures_iter(input)
        .map(|cap| {
            if cap[0].starts_with("don") {
                // eprintln!("Disable with {cap:?}");
                enabled = false;
                0
            } else if cap[0].starts_with("do") {
                // eprintln!("Enable with {cap:?}");
                enabled = true;
                0
            } else if enabled {
                let a: usize = cap[1].parse().unwrap();
                let b: usize = cap[2].parse().unwrap();
                a * b
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day3, part2, speed)]
pub fn part2(input: &str) -> usize {
    let mut chars = input.chars();
    let mut sum = 0;
    let mut counter = 0;
    let mut switchcounter = 0;
    let mut a = 0;
    let mut b = 0;
    let mut enabled = true;
    while let Some(c) = chars.next() {
        match c {
            'd' => {
                if switchcounter == 0 {
                    switchcounter += 1;
                } else {
                    switchcounter = 0;
                }
                counter = 0;
                a = 0;
                b = 0;
            },
            'o' => {
                if switchcounter == 1 {
                    switchcounter += 1;
                } else {
                    switchcounter = 0;

                }
                counter = 0;
                a = 0;
                b = 0;
            },
            'n' => {
                if switchcounter == 2 {
                    switchcounter += 1;
                } else {
                    switchcounter = 0;

                }
                counter = 0;
                a = 0;
                b = 0;
            },
            '\'' => {
                if switchcounter == 3 {
                    switchcounter += 1;
                } else {
                    switchcounter = 0;
                }
                counter = 0;
                a = 0;
                b = 0;
            },
            't' => {
                if switchcounter == 4 {
                    switchcounter += 1;
                } else {
                    switchcounter = 0;
                    counter = 0;
                    a = 0;
                    b = 0;
                }
            },
            'm' => {
                if counter == 0 {
                    counter += 1;
                } else {
                    counter = 1;
                    a = 0;
                    b = 0;
                }
                switchcounter = 0;
            }
            'u' => {
                if counter == 1 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
                switchcounter = 0;
            }
            'l' => {
                if counter == 2 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
                switchcounter = 0;
            }
            '(' => {
                if counter == 3 {
                    counter += 1;
                    switchcounter = 0;
                } else if switchcounter == 2 || switchcounter == 5 {
                    switchcounter += 1;
                    counter = 0;
                    a = 0; 
                    b = 0;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                    switchcounter = 0;
                }
            }
            ',' => {
                if counter == 4 {
                    counter += 1;
                } else {
                    counter = 0;
                    a = 0;
                    b = 0;
                }
                switchcounter = 0;
            }
            ')' => {
                if counter == 5 && enabled{
                    sum += a * b;
                }
                if switchcounter == 3 {
                    enabled = true;
                } else if switchcounter == 6 {
                    enabled = false;
                }
                switchcounter = 0;
                counter = 0;
                a = 0;
                b = 0;
            },
            c => match c.to_digit(10) {
                Some(d) => {
                    if counter == 4 {
                        a = a * 10 + d as usize;
                    } else if counter == 5 {
                        b = b * 10 + d as usize;
                    }
                }
                None => {
                    counter = 0;
                    a = 0;
                    b = 0;
                    switchcounter = 0;
                }
            },
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part1_speed_test() {
        assert_eq!(
            part1_speed("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
    #[test]
    fn part2_example2() {
        assert_eq!(
            part2_speed("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
