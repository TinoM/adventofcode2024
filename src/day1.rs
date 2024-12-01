#[aoc_generator(day1,part1,default)]
pub fn input_generator(input: &str) -> ([u32;1000],[u32;1000]) {
    let mut a = [0;1000];
    let mut b = [0;1000];
    input.lines().enumerate().for_each(|(i,l)| {
        let mut pair = l
        // .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
        a[i] = pair.next().unwrap();
        b[i] = pair.next().unwrap();
    });
    (a, b)
}

#[aoc_generator(day1,part2,default)]
pub fn input_generator2(input: &str) -> ([u32;1000],[u32;1000]) {
    let mut a = [0;1000];
    let mut b = [0;1000];
    input.lines().enumerate().for_each(|(i,l)| {
        let mut pair = l
        // .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
    a[i] = pair.next().unwrap();
    b[i] = pair.next().unwrap();
    });
    (a, b)
}

// #[aoc(day1, part1)]
// pub fn part1_old(input: &Lists) -> u32 {
//     // input.0.sort();
//     input.0
//         .iter()
//         .sorted_unstable()
//         .zip(input.1.iter().sorted_unstable())
//         .map(|(&a,&b)| a.abs_diff(b))
//         .sum()
// }

#[aoc(day1, part1,default)]
pub fn part1_dev(input: &([u32;1000],[u32;1000])) -> u32 {
    let mut a = input.0.clone();
    let mut b = input.1.clone();
    a.sort_unstable();
    b.sort_unstable();
    a
    .iter()
        // .sorted_unstable()
        .zip(b.iter())
        .map(|(a,&b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part1, speed)]
pub fn part1(input:&str) -> u32 {
    let mut a = [0u32;1000];
    let mut b = [0u32;1000];
    input.lines().enumerate().for_each(|(i,l)| {
        let mut pair = l
        // .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
        a[i] = pair.next().unwrap();
        b[i] = pair.next().unwrap();
    });
    a.sort_unstable();
    b.sort_unstable();
    a
    .iter()
        // .sorted_unstable()
        .zip(b.iter())
        .map(|(a,&b)| a.abs_diff(b))
        .sum::<u32>()
}

#[aoc(day1, part2, speed)]
pub fn part2(input:&str) -> u32 {
    let mut a = [0u32;1000];
    // let mut b:Vec<u32,1000> = Vec::new();
    // let mut b:Vec<u32> = Vec::with_capacity(1000);
    let mut counts = [0; 100000];
    input.lines().enumerate().for_each(|(i,l)| {
        let mut pair = l
        // .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
        a[i] = pair.next().unwrap();
        let k = pair.next().unwrap();
        counts[k as usize] += k;
    });
    
    // b.iter().for_each(|&i| counts[i as usize] += i);
    a.iter().map(|&p| counts[p as usize]).sum::<u32>()
}

// #[aoc_generator(day1,part2,speed)]
// pub fn input_generator_speed(input: &str) -> &str {
//     input
// }

#[aoc(day1, part2, default)]
pub fn part2_dev(input: &([u32;1000],[u32;1000])) -> u32 { 
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
            part1_dev(&input_generator(
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
            part2_dev(&input_generator(
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
