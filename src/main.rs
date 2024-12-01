mod day1;
use day1::Day1;

fn main() {
    let day1 = Day1{};
    println!("Day 1 Part 1 : {}",day1.compute("input/day1.txt"));
    println!("Day 1 Part 2 : {}",day1.compute2("input/day1.txt"));
}
