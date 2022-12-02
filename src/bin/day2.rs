use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day2")?;

    let mut sum: u32 = 0;
    input.lines().for_each(|l| {
        let moves: Vec<&str> = l.split_whitespace().take(2).collect();
        let (m1, m2) = (move_to_score(moves[0]), move_to_score(moves[1]));
        sum += calculate_score(m1, m2);
    });
    println!("total score {}", sum);

    let mut sum: u32 = 0;
    input.lines().for_each(|l| {
        let moves: Vec<&str> = l.split_whitespace().take(2).collect();
        let (m1, m2) = (move_to_score(moves[0]), move_to_score(moves[1]));
        let m2 = decode_move(m1, m2);

        sum += calculate_score(m1, m2);
    });
    println!("total decoded score {}", sum);

    Ok(())
}

fn calculate_score(m1: u32, m2: u32) -> u32 {
    // Some cryptic logic
    if m1 == m2 {
        return m2 + 3;
    }

    if (m2 - 1) == m1 || (m2 == 1 && m1 == 3) {
        return m2 + 6;
    }

    return m2;
}

fn move_to_score(m: &str) -> u32 {
    match m {
        "X" | "A" => 1,
        "Y" | "B" => 2,
        "Z" | "C" => 3,
        _ => panic!("unknown move"),
    }
}

fn decode_move(m1: u32, m2: u32) -> u32 {
    // Don't have time to figure out the modulo logic here
    match m2 {
        // Need to lose
        1 => match m1 {
            1 => 3,
            2 => 1,
            3 => 2,
            _ => panic!(),
        },
        // Need to draw
        2 => m1,
        // Need to win
        3 => match m1 {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => panic!(),
        },
        _ => panic!("unknown move"),
    }
}
