use std::collections::HashMap;

use itertools::Itertools;


pub(crate) struct Day1{

}

impl Day1 {
    pub fn compute(&self,file:&str) -> usize {
        println!("Hello from mod.rs");
        let contents = std::fs::read_to_string(file)
            .expect("Should have been able to read the file");
        // println!("File contents: {}", contents);
        let mut a = vec![];
        let mut b = vec![];
        for l in contents.lines() {
            let mut d = l.split_ascii_whitespace();
            a.push(d.next().unwrap().parse::<usize>().unwrap());
            b.push(d.next().unwrap().parse::<usize>().unwrap());
        }
        a.iter().sorted().zip(b.iter().sorted()).map(|(x,y)|x.abs_diff(*y)).sum()
    }

    pub fn compute2(&self,file:&str) -> usize {
        println!("Hello from mod.rs");
        let contents = std::fs::read_to_string(file)
            .expect("Should have been able to read the file");
        // println!("File contents: {}", contents);
        let mut a = vec![];
        let mut b = HashMap::new();
        for l in contents.lines() {
            let mut d = l.split_ascii_whitespace();
            a.push(d.next().unwrap().parse::<usize>().unwrap());
            let key = d.next().unwrap().parse::<usize>().unwrap();
            b.entry(key).and_modify(|count|*count+=1).or_insert(1);
        }
        // eprintln!("{:?}",b);
        a.iter().map(|x|b.get(x).unwrap_or(&0)*x).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_test() {
        let day1 = Day1 {};
        assert_eq!(11, day1.compute("input/test1.txt"));
    }

    #[test]
    fn test_day1_2() {
        let day1 = Day1 {};
        assert_eq!(31, day1.compute2("input/test1.txt"));
    }
}