use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Instruction::Noop),
            s if s.starts_with("addx ") => Ok(Instruction::Addx(
                s.split_whitespace()
                    .nth(1)
                    .ok_or(())?
                    .parse::<i32>()
                    .map_err(|_| ())?,
            )),
            _ => Err(()),
        }
    }
}

struct Machine {
    cycles: usize,
    register_x: i32,
    instructions: Box<dyn Iterator<Item = Instruction>>,
    current_instruction: Option<Instruction>,
    cycles_for_current_instruction: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum MachineRunError {
    EndOfProgram,
    IllegalInstruction,
}

impl Machine {
    fn from_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            cycles: 1,
            register_x: 1,
            instructions: Box::new(instructions.into_iter()),
            current_instruction: None,
            cycles_for_current_instruction: 0,
        }
    }

    fn run_one_cycle(&mut self) -> Result<(), MachineRunError> {
        if self.current_instruction.is_none() {
            self.current_instruction = self.instructions.next();
            let Some(current_instruction) = self.current_instruction else {
                return Err(MachineRunError::EndOfProgram);
            };
            match current_instruction {
                Instruction::Noop => {
                    self.cycles_for_current_instruction = 1;
                }
                Instruction::Addx(_) => {
                    self.cycles_for_current_instruction = 2;
                }
            }
        }

        if self.cycles_for_current_instruction == 0 {
            return Err(MachineRunError::IllegalInstruction);
        }

        self.cycles += 1;
        self.cycles_for_current_instruction -= 1;

        if self.cycles_for_current_instruction == 0 {
            match self.current_instruction {
                None => return Err(MachineRunError::IllegalInstruction),
                Some(Instruction::Noop) => {}
                Some(Instruction::Addx(i)) => self.register_x += i,
            }
            self.current_instruction = None
        }

        Ok(())
    }

    fn run_cycles(&mut self, cycles: usize) -> Result<(), MachineRunError> {
        for _ in 0..cycles {
            self.run_one_cycle()?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_runs() {
        let input = r"noop
addx 3
addx -5";
        let mut machine = Machine::from_instructions(parse(input));
        assert_eq!(machine.cycles, 0);
        machine.run_one_cycle().unwrap();
        assert_eq!(machine.cycles, 1);
        assert_eq!(machine.register_x, 1);
        machine.run_one_cycle().unwrap();
        assert_eq!(machine.cycles, 2);
        assert_eq!(machine.register_x, 1);
        machine.run_one_cycle().unwrap();
        assert_eq!(machine.cycles, 3);
        assert_eq!(machine.register_x, 4);
        machine.run_one_cycle().unwrap();
        assert_eq!(machine.cycles, 4);
        assert_eq!(machine.register_x, 4);
        machine.run_one_cycle().unwrap();
        assert_eq!(machine.cycles, 5);
        assert_eq!(machine.register_x, -1);
    }

    #[test]
    fn it_runs_full() {
        let input = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(part1_run(parse(input)), Ok(13140));
    }
}

fn part1_run(instructions: Vec<Instruction>) -> Result<i32, MachineRunError> {
    let mut machine = Machine::from_instructions(instructions);
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut sum_signal_str = 0;
    while machine.cycles <= *interesting_cycles.last().unwrap() {
        if interesting_cycles.contains(&machine.cycles) {
            println!("{}: {}", machine.cycles, machine.register_x);
            sum_signal_str += machine.cycles as i32 * machine.register_x;
        }
        machine.run_one_cycle()?;
    }
    Ok(sum_signal_str)
}

#[test]
fn part1() {
    let instructions = parse(INPUT);
    println!("{}", part1_run(instructions).unwrap());
}
