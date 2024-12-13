use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let grid:Vec<Vec<u8>> = input.lines().map(|l|l.bytes().collect()).collect();
    let mut glob = HashSet::new();
    let mut result = 0;
    for (y,line) in grid.iter().enumerate() {
        for (x,c) in line.iter().enumerate() {
            
            if !glob.contains(&(x,y)) {
                let mut area = 1;
                let mut perimeter = perim(&(x,y),&grid,c);   
                let mut frontier = vec![(x,y)];
                let mut visited = HashSet::new();
                visited.insert((x,y));
                while let Some((cx,cy)) = frontier.pop() {
                    if cy > 0 {
                        if grid[cy-1][cx] == *c && !visited.contains(&(cx,cy-1)) {
                            area+=1;
                            frontier.push((cx,cy-1));
                            visited.insert((cx,cy-1));
                            perimeter+=perim(&(cx,cy-1), &grid, c);
                        }
                    }
                    if cy < grid.len()-1 {
                        if grid[cy+1][cx] == *c && !visited.contains(&(cx,cy+1)) {
                            area+=1;
                            frontier.push((cx,cy+1));
                            visited.insert((cx,cy+1));
                            perimeter+=perim(&(cx,cy+1), &grid, c);
                        }
                    }
                    if cx > 0 {
                        if grid[cy][cx-1] == *c && !visited.contains(&(cx-1,cy)) {
                            area+=1;
                            frontier.push((cx-1,cy));
                            visited.insert((cx-1,cy));
                            perimeter+=perim(&(cx-1,cy), &grid, c);
                        }
                    }
                    if cx < grid[0].len()-1 {
                        if grid[cy][cx+1] == *c && !visited.contains(&(cx+1,cy)) {
                            area+=1;
                            frontier.push((cx+1,cy));
                            visited.insert((cx+1,cy));
                            perimeter+=perim(&(cx+1,cy), &grid, c);
                        }
                    }
                }
                
                eprintln!("Area {} : {} | Perimeter {}",*c as char,area,perimeter);
                result += area*perimeter;
                glob.extend(visited);
            }

        }
    }
    result
}

fn perim((x,y):&(usize,usize),grid:&Vec<Vec<u8>>,c:&u8) -> usize {
    let mut p = 4;
    if *y > 0 {
        if grid[y-1][*x] == *c {
            p-=1;
        }
    }
    if *y < grid.len()-1 {
        if grid[y+1][*x] == *c {
            p-=1;
        }
    }
    if *x > 0 {
        if grid[*y][x-1] == *c {
            p-=1;
        }
    }
    if *x < grid[0].len()-1 {
        if grid[*y][x+1] == *c {
            p-=1;
        }
    }
    p
}
#[aoc(day12, part2)]
pub fn part2(input: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "<RESULT>");
    }
}