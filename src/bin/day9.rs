use std::collections::HashSet;

use aoc22::input::read_input;

#[derive(Debug)]
struct Move {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day9")?;

    let moves = parse_moves(input);
    simulate_rope(&moves, 2);
    simulate_rope(&moves, 10);

    Ok(())
}

fn simulate_rope(moves: &Vec<Move>, length: usize) {
    let mut knots = vec![(0, 0); length];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(*knots.last().unwrap());
    moves.iter().for_each(|m| {
        for _ in 0..m.x.abs() {
            knots[0].0 += m.x.signum();
            for i in 1..knots.len() {
                knots[i] = move_toward_head(knots[i], knots[i - 1]);
            }
            visited.insert(*knots.last().unwrap());
        }
        for _ in 0..m.y.abs() {
            knots[0].1 += m.y.signum();
            for i in 1..knots.len() {
                knots[i] = move_toward_head(knots[i], knots[i - 1]);
            }
            visited.insert(*knots.last().unwrap());
        }
    });
    println!(
        "unique tail positions for rope of length {}: {}",
        length,
        visited.len()
    );
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

fn parse_moves(input: String) -> Vec<Move> {
    input
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
        .collect()
}
