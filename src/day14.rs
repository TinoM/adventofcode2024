
use aoc_runner_derive::aoc;
use regex::Regex;

const WIDTH:i32 = 101;
const HEIGHT:i32 = 103;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut q1,mut q2,mut q3,mut q4) = (0,0,0,0);
    let mut sign = 1;
    let mut vals = [0;4];
    let mut current_digit = 0;
    eprintln!("Linebreaks: {}",input.bytes().filter(|&b| b == 10).count());
    for b in input.bytes() {
        match b {
            b'-' => sign = -1,
            10 => {
                //reset
                vals[current_digit]*=sign;
                // eprintln!("Byteparser: {:?}", vals);
                if vals[2] < 0 {
                    vals[2] = WIDTH as i32+vals[2];
                }
                if vals[3] < 0 {
                    vals[3] = HEIGHT as i32+vals[3];
                }
                vals[0] = (vals[0] + (100 * vals[2])) % WIDTH;
                vals[1] = (vals[1] + (100 * vals[3])) % HEIGHT;
                // eprintln!("{} {} {} {}", x, y,vx,vy);
                if vals[0] < WIDTH/2 && vals[1] < HEIGHT/2 {
                    q1 += 1;
                } else if vals[0] > WIDTH/2 && vals[1] < HEIGHT/2 {
                    q2 += 1;
                } else if vals[0] < WIDTH/2 && vals[1] > HEIGHT/2 {
                    q3 += 1;
                } else if vals[0] > WIDTH/2 && vals[1] > HEIGHT/2 {
                    q4 += 1;
                }
                sign = 1;
                current_digit = 0;
                vals = [0;4];
            },
            b','|b' ' => {
                vals[current_digit]*=sign;
                current_digit += 1;
                sign = 1;
            },
            b'p'|b'v'|b'='|13 => {},
            v => {
                // eprintln!("{:?}",vals);
                vals[current_digit] = vals[current_digit] * 10 + (v - b'0') as i32;
                // eprintln!("{} {} {} {}", current_digit, v as char,vals[current_digit],(v - b'0') as i32);
            }
        }   
    }
    vals[current_digit]*=sign;
    if vals[2] < 0 {
        vals[2] = WIDTH as i32+vals[2];
    }
    if vals[3] < 0 {
        vals[3] = HEIGHT as i32+vals[3];
    }
    vals[0] = (vals[0] + (100 * vals[2])) % WIDTH;
    vals[1] = (vals[1] + (100 * vals[3])) % HEIGHT;
    // eprintln!("{} {} {} {}", x, y,vx,vy);
    if vals[0] < WIDTH/2 && vals[1] < HEIGHT/2 {
        q1 += 1;
    } else if vals[0] > WIDTH/2 && vals[1] < HEIGHT/2 {
        q2 += 1;
    } else if vals[0] < WIDTH/2 && vals[1] > HEIGHT/2 {
        q3 += 1;
    } else if vals[0] > WIDTH/2 && vals[1] > HEIGHT/2 {
        q4 += 1;
    }
    // eprintln!("{vals:?} {:?}",input.bytes().rev().take(30).collect::<Vec<u8>>());    
    // eprintln!("Q1: {} Q2: {} Q3: {} Q4: {}", q1, q2, q3, q4);
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    const WIDTH:i32 = 101;
    const HEIGHT:i32 = 103;
    let mut robots = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            let mut vx = cap[3].parse::<i32>().unwrap();
            let mut vy = cap[4].parse::<i32>().unwrap();
            if vx < 0 {
                vx = WIDTH as i32+vx;
            }
            if vy < 0 {
                vy = HEIGHT as i32+vy;
            }
            (x, y, vx, vy)
        })
        .collect::<Vec<(i32, i32, i32, i32)>>();
    'second: for second in 1..=WIDTH * HEIGHT {
        for r in robots.iter_mut() {
            r.0 = (r.0 + (1 * r.2)) % WIDTH;
            r.1 = (r.1 + (1 * r.3)) % HEIGHT;
        }
        let mut set = [false;WIDTH as usize*HEIGHT as usize];
        for r in robots.iter() {
            // set.entry((r.0, r.1)).and_modify(|c| *c += 1).or_insert(1);
            if set[(r.0 + r.1 * WIDTH) as usize] {
                continue 'second;
            } else {
                set[(r.0 + r.1 * WIDTH) as usize] = true;
            }
        }
        return second as i32;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 12);
    }

}
