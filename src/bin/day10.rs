use aoc22::input::read_input;

enum Instruction {
    AddX(i32),
    NoOp,
}
struct Cpu {
    x: i32,
    // Register used as flag for executing addx over multiple cycles
    y: i32,
    // Program counter keeps track of where we are in execution
    pc: usize,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { x: 1, y: 0, pc: 0 }
    }

    fn run(&mut self, instructions: &Vec<Instruction>, mut execute_cycle: impl FnMut(&mut Cpu)) {
        while self.pc < instructions.len() {
            execute_cycle(self);

            let instruction = &instructions[self.pc];
            match instruction {
                Instruction::AddX(val) => {
                    if self.y == 1 {
                        self.x += val;
                        self.y = 0;
                        self.pc += 1;
                    } else {
                        self.y = 1;
                    }
                }
                Instruction::NoOp => self.pc += 1,
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day10")?;
    let instructions = read_instructions(input);

    // Part 1
    let mut cpu1 = Cpu::new();
    let mut signal_strength = 0;
    let mut cycles = 0;
    cpu1.run(&instructions, |cpu| {
        cycles += 1;
        if cycles == 20
            || cycles == 60
            || cycles == 100
            || cycles == 140
            || cycles == 180
            || cycles == 220
        {
            signal_strength += cycles * cpu.x;
        }
    });
    println!("signal strength is {}", signal_strength);

    // Part 2
    let mut cpu2 = Cpu::new();
    let mut col = 0;
    cpu2.run(&instructions, |cpu| {
        if cpu.x.abs_diff(col) <= 1 {
            print!("{}", '#');
        } else {
            print!("{}", '.');
        }
        col += 1;
        if col >= 40 {
            print!("\n");
            col = 0;
        }
    });

    Ok(())
}

fn read_instructions(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            match parts[0] {
                "addx" => Instruction::AddX(parts[1].parse::<i32>().unwrap()),
                "noop" => Instruction::NoOp,
                _ => panic!("unknown instruction {}", parts[0]),
            }
        })
        .collect()
}
