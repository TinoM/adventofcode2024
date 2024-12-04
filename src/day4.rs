use aoc_runner_derive::aoc;

const SEARCHWORD: [char; 4] = ['X', 'M', 'A', 'S'];

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sol = 0;

    for (y, l) in grid.iter().enumerate() {
        for (x, _) in l.iter().enumerate() {
            //Right
            if x < grid[0].len() - 3 {
                if (0..4).all(|xd| {
                    grid[y][xd+x] == SEARCHWORD[xd]
                }) {
                    sol += 1;
                    // eprintln!("Hit R at {},{}", x, y);
                };
            }
            //Down
            if y < grid.len() - 3 {
                if (0..4).all(|yd| {
                    grid[y+yd][x] == SEARCHWORD[yd]
                }) {
                    sol += 1;
                    // eprintln!("Hit D at {},{}", x, y);
                };
            }
            //Left
            if x > 2 {
                if (0..4).all(|xd| {
                    grid[y][x-xd] == SEARCHWORD[xd]
                }) {
                    sol += 1;
                    // eprintln!("Hit L at {},{}", x, y);
                };
            }
            //Up
            if y > 2 {
                if (0..4).all(|yd| {
                    grid[y-yd][x] == SEARCHWORD[yd]
                }) {
                    sol += 1;
                    // eprintln!("Hit U at {},{}", x, y);
                };
            }
            //UL
            if x > 2 && y > 2 {
                if (0..4).all(|d| {
                    grid[y-d][x-d] == SEARCHWORD[d]
                }) {
                    sol += 1;
                    // eprintln!("Hit UL at {},{}", x, y);
                };
            }
            //UR
            if x < grid[0].len()-3 && y > 2 {
                if (0..4).all(|d| {
                    grid[y-d][x+d] == SEARCHWORD[d]
                }) {
                    sol += 1;
                    // eprintln!("Hit UR at {},{}", x, y);
                };
            }
            //DR
            if x < grid[0].len()-3 && y < grid.len()-3 {
                if (0..4).all(|d| {
                    grid[y+d][x+d] == SEARCHWORD[d]
                }) {
                    sol += 1;
                    // eprintln!("Hit DR at {},{}", x, y);
                };
            }
            //DL
            if x > 2 && y < grid.len()-3 {
                if (0..4).all(|d| {
                    grid[y+d][x-d] == SEARCHWORD[d]
                }) {
                    sol += 1;
                    // eprintln!("Hit DL at {},{}", x, y);
                };
            }
        }
    }
    sol
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<char> = input.chars().filter(|c|c.is_alphabetic()).collect();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    // eprintln!("{} {}",height,width);
    let mut sol = 0;
    for y in 0..height-2 {
        for x in 0..width-2 {
                // eprintln!("{} {}",x,y); 
                let one = &grid[y*width+x..=y*width+x+2];
                let two = &grid[(y+1)*width+x..=(y+1)*width+x+2];
                let three = &grid[(y+2)*width+x..=(y+2)*width+x+2];

                if two[1] == 'A' && ((one[0] == 'M' && three[2] == 'S') || one[0] == 'S' && three[2] == 'M') && ((one[2] == 'M' && three[0] == 'S') || one[2] == 'S' && three[0] == 'M') {
                    sol += 1;
                    // eprintln!("{:?}",one);
                    // eprintln!("{two:?}");
                    // eprintln!("{three:?}");
                }
        }
    }
    sol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            18
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            9
        );
    }
}
