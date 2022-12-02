use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day2")?;

    let mut sum: u32 = 0;
    input.lines().for_each(|l| {
        let moves: Vec<&str> = l.split_whitespace().take(2).collect();
        let (m1, m2) = (moves[0], moves[1]);
        sum += calculate_score(m1, m2);
    });

    println!("total score {}", sum);
    Ok(())
}

fn calculate_score(m1: &str, m2: &str) -> u32 {
    let (m1, m2) = (move_to_score(m1), move_to_score(m2));

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
    return match m {
        "X" | "A" => 1,
        "Y" | "B" => 2,
        "Z" | "C" => 3,
        _ => panic!("unknown move"),
    };
}
