use std::collections::HashSet;

use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day6")?;
    let chars = input.chars().collect::<Vec<char>>();

    let start_of_packet_index = first_distinct_window(&chars, 4);
    println!("start of packet index {}", start_of_packet_index);

    let start_of_message_index = first_distinct_window(&chars, 14);
    println!("start of message index {}", start_of_message_index);
    Ok(())
}

fn first_distinct_window(chars: &Vec<char>, n: usize) -> usize {
    let mut start_index = 0;

    // Iterate over all overlapping windows of size `n`
    let windows = chars.windows(n).collect::<Vec<&[char]>>();
    for (i, window) in windows.iter().cloned().enumerate() {
        // Collect the whole window into a set to detect duplicates.
        let set = window.iter().cloned().collect::<HashSet<char>>();
        if set.len() == n {
            start_index = i + n;
            break;
        }
    }

    start_index
}
