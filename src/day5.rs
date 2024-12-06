use std::collections::HashMap;

use itertools::Itertools;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut solution = 0;
    let mut dic:HashMap<u8, Vec<u8>> = HashMap::new();
    while let Some(line) = lines.next() {
        // line.split("|").flat_map(|d|d.parse::<u8>()).tuples().for_each(|(a,b)| {
        //     dic.entry(a).and_modify(|d|d.push(b)).or_insert(vec![b]);    
        // });
        if line.is_empty() {
            break;
        }
        let mut tmp = [0,0];
        let mut i = 0;
        for  b in line.bytes() {
            if b == b'|' {
                i+=1;
                continue;
            } else {
                // eprintln!("{} => {} {:?}",line,i,tmp);
                tmp[i] = tmp[i]*10 +(b-48) as u8;
            }
        }
        // eprintln!("{} => {:?}",line,tmp);
        dic.entry(tmp[0]).and_modify(|d|d.push(tmp[1])).or_insert(vec![tmp[1]]);
    }
    // eprintln!("{:?}",dic);
    solution += lines.map(|line|{
        let c = check(line,&dic);
        // eprintln!("{} -> {}",line,c);
        c
    }).sum::<usize>();
    solution
}

fn check(input: &str,dic:&HashMap<u8,Vec<u8>>) -> usize {
    // let nums:Vec<u8> = input.split(",").flat_map(|d|d.parse::<u8>()).collect();
    let mut nums = vec![];
    let mut tmp = 0;
    for c in input.bytes() {
        if c == b',' {
            nums.push(tmp);
            tmp = 0;
            continue;
        } else {
            tmp = tmp*10 +(c-48) as u8;
        }
    }
    nums.push(tmp);
    if nums.iter().tuple_windows().all(|(a,b)|dic.get(a).map_or(false,|d|d.contains(b))) {
        nums[nums.len()/2] as usize
    } else {
        0
    }
}

fn check2(input: &str,dic:&HashMap<u8,Vec<u8>>) -> usize {
    let mut nums = vec![];
    let mut tmp = 0;
    for c in input.bytes() {
        if c == b',' {
            nums.push(tmp);
            tmp = 0;
            continue;
        } else {
            tmp = tmp*10 +(c-48) as u8;
        }
    }
    nums.push(tmp);
    if nums.iter().tuple_windows().all(|(a,b)|dic.get(a).map_or(false,|d|d.contains(b))) {
        0
    } else {
        nums.sort_by(|a,b|{
            if let Some(k) = dic.get(a) {
                if k.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            } else if let Some(k) = dic.get(b) {
                if k.contains(a) {
                    return std::cmp::Ordering::Greater;
                }
            }
            std::cmp::Ordering::Equal
        });
        nums[nums.len()/2] as usize
    }
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut solution = 0;
    let mut dic:HashMap<u8, Vec<u8>> = HashMap::new();
    while let Some(line) = lines.next() {
        // line.split("|").flat_map(|d|d.parse::<u8>()).tuples().for_each(|(a,b)| {
        //     dic.entry(a).and_modify(|d|d.push(b)).or_insert(vec![b]);    
        // });
        if line.is_empty() {
            break;
        }
        let mut tmp = [0,0];
        let mut i = 0;
        for  b in line.bytes() {
            if b == b'|' {
                i+=1;
                continue;
            } else {
                // eprintln!("{} => {} {:?}",line,i,tmp);
                tmp[i] = tmp[i]*10 +(b-48) as u8;
            }
        }
        // eprintln!("{} => {:?}",line,tmp);
        dic.entry(tmp[0]).and_modify(|d|d.push(tmp[1])).or_insert(vec![tmp[1]]);
    }
    // eprintln!("{:?}",dic);
    solution += lines.map(|line|{
        let c = check2(line,&dic);
        // eprintln!("{} -> {}",line,c);
        c
    }).sum::<usize>();
    solution
}



#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA:&str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_opt() {
        assert_eq!(part1(TESTDATA), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TESTDATA), 123);
    }
}