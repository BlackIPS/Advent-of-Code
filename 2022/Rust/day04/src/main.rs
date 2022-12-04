use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

mod tests;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("could not load input file");
    let content: Vec<_> = input.lines().collect();

    println!("Step 1 result: {}", process_part1(content.clone()));
    println!("Step 2 result: {}", process_part2(content.clone()));
}


fn process_part1(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|l| {
            let line = Line::from_str(l).unwrap();

            if line.left < line.right {
                line.left.clamp(line.right.start,line.right.end)
            } else {
                line.right.clamp(line.left.start,line.left.end)
            }
        })
        .map(|res| if res { 1 } else { 0 })
        .sum::<i32>()
}


fn process_part2(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|l| {
            let line = Line::from_str(l).unwrap();

            if line.left < line.right {
                line.left.overlap(line.right)
            } else {
                line.right.overlap(line.left)
            }
        })
        .map(|res| if res { 1 } else { 0 })
        .sum::<i32>()
}


#[derive(Debug, PartialEq)]
struct Line {
    left: Section,
    right: Section
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(",").unwrap();

        let first_section = Section::from_str(first).unwrap();
        let second_section = Section::from_str(second).unwrap();

        Ok(Self { left: first_section, right: second_section})
    }
}


#[derive(Debug,PartialEq,Copy, Clone)]
struct Section {
    start: i32,
    end: i32,
}

impl FromStr for Section {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x,y) = s.split_once("-").unwrap();

        let start = x.parse::<i32>()?;
        let end = y.parse::<i32>()?;

        Ok(Self { start, end })
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(&other) { return Some(Ordering::Less) }
        if self.gt(&other) { return Some(Ordering::Greater) }

        None
    }

    fn lt(&self, other: &Self) -> bool {
        (self.end - self.start) < (other.end - other.start)
    }

    fn gt(&self, other: &Self) -> bool {
        (self.end - self.start) > (other.end - other.start)
    }
}

impl Section {
    fn clamp(self, min: i32, max: i32) -> bool {
        assert!(min <= max);

        self.start >= min && self.end <= max
    }

    fn overlap(self, other: Self) -> bool {
        if self.clamp(other.start.clone(), other.end.clone()) { return true; }
        if self.start >= other.start && self.start <= other.end { return true; }
        if self.end >= other.start && self.end <= other.end { return true; }

        false
    }
}