use std::collections::VecDeque;

use aoc22::input::read_input;
use regex::Regex;

struct Move {
    count: u32,
    from: usize,
    to: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day5")?;

    let mut stacks = read_stacks(input.clone());
    let moves: Vec<Move> = input
        .lines()
        .filter(|l| l.starts_with("move"))
        .map(|l| {
            let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
            let cs = re.captures(l).unwrap();
            Move {
                count: cs.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                from: cs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: cs.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Move>>();

    for m in moves {
        for _ in 0..m.count {
            let char = stacks[m.from - 1].pop_front().unwrap();
            stacks[m.to - 1].push_front(char);
        }
    }

    println!(
        "{}",
        stacks.iter().map(|s| s.get(0).unwrap()).collect::<String>()
    );

    Ok(())
}

fn read_stacks(input: String) -> Vec<VecDeque<char>> {
    let stack_lines: Vec<&str> = input.lines().take_while(|l| l.starts_with("[")).collect();
    let mut stacks: Vec<VecDeque<char>> = vec![];
    stack_lines.iter().for_each(|l| {
        for i in (0..l.len()).step_by(4) {
            let stack = i / 4;
            if let None = stacks.get(stack) {
                stacks.push(VecDeque::new());
            }

            if l.chars().nth(i).unwrap() == '[' {
                stacks[stack].push_back(l.chars().nth(i + 1).unwrap());
            }
        }
    });
    stacks
}
