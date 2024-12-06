use std::collections::HashSet;
use num::complex::Complex;


#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut pos:Complex<i32> = Complex::new(0, 0);
    
    let grid:Vec<Vec<u8>> = input.lines().enumerate().map(|(y,l)|{
        l.bytes().enumerate().map(|(x,b)|{
            if b == b'^' {
                pos.re = x as i32;
                pos.im = y as i32;
            }
            b
        }).collect()
    }).collect();
    let mut dir = Complex::new(0, -1);
    let mut visited = vec![false;grid.len()*grid[0].len()];
    loop {
        visited[pos.im as usize * grid[0].len() + pos.re as usize] = true;
        let new_pos = pos + dir;
        if new_pos.re < 0 || new_pos.im < 0 || new_pos.im as usize == grid.len() || new_pos.re as usize == grid[0].len() {
            break;
        }
        if grid[new_pos.im as usize][new_pos.re as usize] != b'#' {
            pos = new_pos;
        } else {
            dir *= Complex::new(0, 1);
        }
    }
    visited.iter().filter(|&&v|v).count()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut pos:Complex<i32> = Complex::new(0, 0);
    let mut dir = Complex::new(0, -1);
    let mut grid:Vec<Vec<u8>> = input.lines().enumerate().map(|(y,l)|{
        l.bytes().enumerate().map(|(x,b)|{
            if b == b'^' {
                pos.re = x as i32;
                pos.im = y as i32;
            }
            b
        }).collect()
    }).collect();
    let w = grid[0].len();
    let h = grid.len();
    let mut visited = vec![false;grid.len()*grid[0].len()];
    let origin = pos;
    pos += dir;
    loop {
        // eprintln!("Guard at {},{} facing {:?}", pos.re, pos.im, dir);
        visited[pos.im as usize * grid[0].len() + pos.re as usize] = true;
        let new_pos = pos + dir;
        if new_pos.re < 0 || new_pos.im < 0 || new_pos.im as usize == grid.len() || new_pos.re as usize == grid[0].len() {
            break;
        }
        
        if grid[new_pos.im as usize][new_pos.re as usize] != b'#' {
            pos = new_pos;
        } else {
            dir *= Complex::new(0, 1);
        }
    }
    let mut loops = 0;
    for block in visited.iter().enumerate().filter(|x|*x.1) {
        // eprintln!("Blocking {:?}", block);
        grid[block.0 / h][block.0 % w] = b'#';
        let mut vis2 = HashSet::new();
        let mut pos = origin;
        let mut dir = Complex::new(0, -1);
        vis2.insert((origin,Complex::new(0, -1)));
        loop {

            // eprintln!("Guard at {},{} facing {:?}", pos.re, pos.im, dir);
            let new_pos = pos + dir;
            if new_pos.re < 0 || new_pos.im < 0 || new_pos.im as usize == grid.len() || new_pos.re as usize == grid[0].len() {
                break;
            }
            if grid[new_pos.im as usize][new_pos.re as usize] != b'#' {
                pos = new_pos;
            } else {
                dir *= Complex::new(0, 1);
            }
            if !vis2.insert((pos,dir)) {
                // eprintln!("Loop detected");
                loops+=1;
                break;
            }
        }
        grid[block.0 / h][block.0 % w] = b'.';
    }
    loops
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT:&str = "....#.....\n\
                        .........#\n\
                        ..........\n\
                        ..#.......\n\
                        .......#..\n\
                        ..........\n\
                        .#..^.....\n\
                        ........#.\n\
                        #.........\n\
                        ......#...";

    #[test]
    fn day_6part1_example() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn day6part2_example() {
        assert_eq!(part2(INPUT), 6);
    }
}