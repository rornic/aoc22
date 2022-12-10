use aoc22::input::read_input;

enum Instruction {
    AddX(i32),
    NoOp,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day10")?;

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            match parts[0] {
                "addx" => Instruction::AddX(parts[1].parse::<i32>().unwrap()),
                "noop" => Instruction::NoOp,
                _ => panic!("unknown instruction {}", parts[0]),
            }
        })
        .collect();

    let mut signal_strength = 0;

    // x register stores the signal strength value
    let mut x = 1;

    // y register is used as a flag for when we are executing an addx instruction
    let mut y = 0;

    let mut cycles = 1;
    let mut program_counter = 0;
    while program_counter < instructions.len() {
        // Refactor this in part 2
        if cycles == 20
            || cycles == 60
            || cycles == 100
            || cycles == 140
            || cycles == 180
            || cycles == 220
        {
            signal_strength += cycles * x;
        }

        let instruction = &instructions[program_counter];
        match instruction {
            Instruction::AddX(val) => {
                if y == 1 {
                    x += val;
                    y = 0;
                    program_counter += 1;
                } else {
                    y = 1;
                }
            }
            Instruction::NoOp => program_counter += 1,
        }
        cycles += 1;
        println!("cycle {}: {}", cycles, x);
    }

    println!("signal strength is {}", signal_strength);

    Ok(())
}
