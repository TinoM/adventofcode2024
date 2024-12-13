use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let mut lit = input.lines();
    let button = Regex::new(r"Button ([A-Z]): X\+(\d+), Y\+(\d+)").unwrap();
    let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut result = 0;
    loop {
        let a = lit.next().unwrap();
        let aa = button.captures(a).unwrap();
        let b = lit.next().unwrap();
        let bb = button.captures(b).unwrap();
        let c = lit.next().unwrap();
        let cc = prize.captures(c).expect(c.to_string().as_str());
        // eprintln!("{:?} {:?} {:?}", aa, bb, cc);
        let buttonax = aa.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let buttonay = aa.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let buttonbx = bb.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let buttonby = bb.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let prizex = cc.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let prizey = cc.get(2).unwrap().as_str().parse::<i32>().unwrap();
        // A = (p_x*b_y - prize_y*b_x) / (a_x*b_y - a_y*b_x)
        // B = (a_x*p_y - a_y*p_x) / (a_x*b_y - a_y*b_x)
        let det = buttonax*buttonby - buttonay*buttonbx;
        let a = (prizex*buttonby - prizey*buttonbx) / det;
        let b = (buttonax*prizey - buttonay*prizex) / det;
        // eprintln!("{} {} | {} {}", a, b,prizex, prizey);
        if a*buttonax + b*buttonbx == prizex && a*buttonay + b*buttonby == prizey {
            result += 3*a+b;
        }
        if lit.next().is_some() {
            continue
        } else {
            break
        }
    }
    result
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let offset:i64 = 10000000000000;
    let mut lit = input.lines();
    let button = Regex::new(r"Button ([A-Z]): X\+(\d+), Y\+(\d+)").unwrap();
    let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut result = 0;
    loop {
        let a = lit.next().unwrap();
        let aa = button.captures(a).unwrap();
        let b = lit.next().unwrap();
        let bb = button.captures(b).unwrap();
        let c = lit.next().unwrap();
        let cc = prize.captures(c).expect(c.to_string().as_str());
        // eprintln!("{:?} {:?} {:?}", aa, bb, cc);
        let buttonax = aa.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let buttonay = aa.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let buttonbx = bb.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let buttonby = bb.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let prizex = cc.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let prizey = cc.get(2).unwrap().as_str().parse::<i64>().unwrap();
        // A = (p_x*b_y - prize_y*b_x) / (a_x*b_y - a_y*b_x)
        // B = (a_x*p_y - a_y*p_x) / (a_x*b_y - a_y*b_x)
        let det = buttonax*buttonby - buttonay*buttonbx;
        let a = ((prizex+offset)*buttonby - (prizey+offset)*buttonbx) / det;
        let b = (buttonax*(prizey+offset) - buttonay*(prizex+offset)) / det;
        // eprintln!("{} {} | {} {}", a, b,prizex, prizey);
        if a*buttonax + b*buttonbx == prizex+offset && a*buttonay + b*buttonby == prizey+offset {
            result += 3*a+b;
        }
        if lit.next().is_some() {
            continue
        } else {
            break
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 480);
    }
}