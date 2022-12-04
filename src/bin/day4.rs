use std::ops::{Range, RangeInclusive};

use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day4")?;

    let mut count = 0;
    input.lines().for_each(|line| {
        let (range1, range2) = line_to_ranges(line);

        let (x1, x2) = (range1.clone().min(), range1.max());
        let (y1, y2) = (range2.clone().min(), range2.max());
        if (x1 >= y1 && x2 <= y2) || (y1 >= x1 && y2 <= x2) {
            count += 1;
        }
    });
    println!("overlapping count {}", count);

    Ok(())
}

fn line_to_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let mut ranges = line.split(",");
    (
        parse_range(ranges.next().unwrap()),
        parse_range(ranges.next().unwrap()),
    )
}

fn parse_range(range: &str) -> RangeInclusive<u32> {
    let mut parts = range.split("-");
    parts.next().unwrap().parse::<u32>().unwrap()..=parts.next().unwrap().parse::<u32>().unwrap()
}
