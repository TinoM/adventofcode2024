use itertools::Itertools;

type Pair = (usize,usize);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let mut pair = l
                .trim()
                .split_ascii_whitespace()
                .map(|d| d.parse().unwrap());
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Pair]) -> usize {
    input
        .iter()
        .sorted_by_key(|p| p.0)
        .zip(input.iter().sorted_by_key(|p| p.1))
        .map(|x| x.0 .0.abs_diff(x.1 .1))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Pair]) -> usize {
    let mut counts = [0;100000];
    input.iter().for_each(|i|counts[i.1]+=1);
    input
        .iter()
        .map(|p| p.0 * counts[p.1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
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
        assert_eq!(solve_part2(&input_generator(                "3   4
        4   3
        2   5
        1   3
        3   9
        3   3")), 31);
    }
}
