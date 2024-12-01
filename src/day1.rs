use itertools::Itertools;

type Lists = (Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Lists {
    let mut a = Vec::with_capacity(1000);
    let mut b = Vec::with_capacity(1000);
    input.lines().for_each(|l| {
        let mut pair = l
        // .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
        a.push(pair.next().unwrap());
        b.push(pair.next().unwrap());
    });
    (a, b)
}

#[aoc(day1, part1)]
pub fn part1(input: &Lists) -> u32 {
    // input.0.sort();
    input.0
        .iter()
        .sorted_unstable()
        .zip(input.1.iter().sorted_unstable())
        .map(|(&a,&b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &Lists) -> u32 { 
    let mut counts = [0; 100000];
    input.1.iter().for_each(|&i| counts[i as usize] += i);
    input.0.iter().map(|&p| counts[p as usize]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            11
        );
    }

    #[test]
    // A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "3   4
        4   3
        2   5
        1   3
        3   9
        3   3"
            )),
            31
        );
    }
}
