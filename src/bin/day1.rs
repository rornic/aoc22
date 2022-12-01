use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day1")?;

    let elves: Vec<u32> = input.lines().fold(vec![0], |elves, line| {
        let mut new_elves = elves.clone();
        if line.is_empty() {
            new_elves.push(0);
        } else {
            *new_elves.last_mut().unwrap() +=
                line.parse::<u32>().expect("line did not contain u32");
        }
        new_elves
    });

    println!("total calories {}", elves.iter().max().unwrap());

    Ok(())
}
