use std::collections::HashSet;

use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day6")?;

    let chars = input.chars().collect::<Vec<char>>();
    let windows = chars.windows(4).collect::<Vec<&[char]>>();

    let mut start_index = 0;
    for (i, window) in windows.iter().cloned().enumerate() {
        let set = window.iter().cloned().collect::<HashSet<char>>();
        if set.len() == 4 {
            start_index = i + 4;
            break;
        }
    }

    println!("start of packet index {}", start_index);
    Ok(())
}
