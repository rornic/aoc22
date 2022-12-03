use std::collections::HashSet;

use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day3")?;

    let total_sum: u64 = input
        .lines()
        .into_iter()
        .map(|line| {
            let c1: HashSet<char> = line.chars().take(line.len() / 2).collect();
            let c2: HashSet<char> = line.chars().skip(line.len() / 2).collect();
            let intersect: HashSet<&char> = c1.intersection(&c2).collect();
            intersect
                .into_iter()
                .map(|c| item_to_priority(*c))
                .sum::<u64>()
        })
        .sum();
    println!("total common item sum {}", total_sum);

    Ok(())
}

fn item_to_priority(item: char) -> u64 {
    let ascii_code = item as u64;
    if ascii_code >= 97 {
        return ascii_code - 96;
    }

    return ascii_code - 38;
}

#[cfg(test)]
mod tests {
    use crate::item_to_priority;

    #[test]
    fn test_priority() {
        assert_eq!(16, item_to_priority('p'));
        assert_eq!(38, item_to_priority('L'));
        assert_eq!(42, item_to_priority('P'));
        assert_eq!(22, item_to_priority('v'));
        assert_eq!(20, item_to_priority('t'));
        assert_eq!(19, item_to_priority('s'));
    }
}
