use std::collections::HashSet;

use aoc22::input::read_input;

#[derive(Debug)]
struct Move {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day9")?;

    let moves: Vec<Move> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let mut val = parts[1].parse::<i32>().unwrap();
            if parts[0] == "D" || parts[0] == "L" {
                val = -val;
            }

            if parts[0] == "L" || parts[0] == "R" {
                Move { x: val, y: 0 }
            } else {
                Move { x: 0, y: val }
            }
        })
        .collect();

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);
    moves.iter().for_each(|m| {
        for _ in 0..m.x.abs() {
            head.0 += m.x.signum();
            tail = move_toward_head(tail, head);
            visited.insert(tail);
        }
        for _ in 0..m.y.abs() {
            head.1 += m.y.signum();
            tail = move_toward_head(tail, head);
            visited.insert(tail);
        }
    });
    println!("unique tail positions {}", visited.len());

    Ok(())
}

fn move_toward_head(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    if x_diff.abs() == 2 && head.1 == tail.1 {
        return (tail.0 + x_diff.signum(), tail.1);
    }

    if y_diff.abs() == 2 && head.0 == tail.0 {
        return (tail.0, tail.1 + y_diff.signum());
    }

    if x_diff.abs() + y_diff.abs() > 2 && (head.0 != tail.0 && head.1 != tail.1) {
        return (tail.0 + x_diff.signum(), tail.1 + y_diff.signum());
    }

    return tail;
}
