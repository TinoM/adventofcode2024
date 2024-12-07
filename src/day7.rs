use itertools::Itertools;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().flat_map(|line|check(line)).sum()
}

fn check(line:&str) -> Result<usize,()> {
    let (n,l) = line.split(":").tuple_windows().next().unwrap();
    let result = n.parse::<usize>().unwrap();
    let numbers:Vec<usize> = l.split_ascii_whitespace().flat_map(|n|n.parse()).collect();
    if numbers.iter().skip(1).fold(vec![numbers[0]],|acc, x|{
        acc.iter().map(|&a|a+x).chain(acc.iter().map(|&a|a*x)).filter(|&n|n<=result).collect()
    }).iter().any(|&res|res == result) {
        Ok(result)
    } else {
        Err(())
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().flat_map(|line|check2(line)).sum()
}

fn check2(line:&str) -> Result<usize,()> {
    let (n,l) = line.split(":").tuple_windows().next().unwrap();
    let result = n.parse::<usize>().unwrap();
    let numbers:Vec<usize> = l.split_ascii_whitespace().flat_map(|n|n.parse()).collect();
    if numbers.iter().skip(1).fold(vec![numbers[0]],|acc, x|{
        acc.iter().map(|&a|a+x).chain(acc.iter().map(|&a|a*x)).chain(
            acc.iter().flat_map(|&a|{
                format!("{}{}",a,x).parse()
            })
        ).filter(|&n|n<=result).collect()
    }).iter().any(|&res|res == result) {
        Ok(result)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 11387);
    }
}