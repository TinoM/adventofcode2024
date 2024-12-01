#[aoc(day1, part1, speed)]
pub fn part1(input: &str) -> u32 {
    let mut a = [0u32; 1000];
    let mut b = [0u32; 1000];
    input.lines().enumerate().for_each(|(i, l)| {
        let mut pair = l
            // .trim()
            .split_ascii_whitespace()
            .map(|d| d.parse().unwrap());
        a[i] = pair.next().unwrap();
        b[i] = pair.next().unwrap();
    });
    a.sort_unstable();
    b.sort_unstable();
    a.iter()
        .zip(b.iter())
        .map(|(a, &b)| a.abs_diff(b))
        .sum::<u32>()
}

#[aoc(day1, part2, speed)]
pub fn part2(input: &str) -> u32 {
    let mut a = [0u32; 1000];
    let mut counts = [0; 100000];
    input.lines().enumerate().for_each(|(i, l)| {
        let mut pair = l
            // .trim()
            .split_ascii_whitespace()
            .map(|d| d.parse().unwrap());
        a[i] = pair.next().unwrap();
        let k = pair.next().unwrap();
        counts[k as usize] += k;
    });
    a.iter().map(|&p| counts[p as usize]).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(
            part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        );
    }

    #[test]
    // A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
    fn example2() {
        assert_eq!(
            part2(
                "3   4
        4   3
        2   5
        1   3
        3   9
        3   3"
            ),
            31
        );
    }
}
