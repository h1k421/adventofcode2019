use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct IntMachine {
    ip: usize,
    ram: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq)]
enum MachineError {
    Exiting,
    InvalidInstruction(u64),
    OutOfBound(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum InstructionType {
    Addition,
    Multiplication,
    Exit,
}

impl InstructionType {
    pub fn from_opcode(opcode: u64) -> Result<InstructionType, MachineError> {
        match opcode {
            1 => Ok(InstructionType::Addition),
            2 => Ok(InstructionType::Multiplication),
            99 => Ok(InstructionType::Exit),
            _ => Err(MachineError::InvalidInstruction(opcode)),
        }
    }
}

#[derive(Debug)]
struct InstructionContext {
    instruction: InstructionType,
    arguments: [u64; 2],
    result_position: usize,
}

impl IntMachine {
    pub fn new(ram: Vec<u64>) -> Self {
        IntMachine { ip: 0, ram }
    }

    fn read_instruction(&mut self) -> Result<InstructionContext, MachineError> {
        if self.ram.len() <= self.ip {
            return Err(MachineError::OutOfBound(self.ip));
        }

        let instruction = InstructionType::from_opcode(self.ram[self.ip])?;

        if instruction == InstructionType::Exit {
            return Err(MachineError::Exiting);
        }

        let result = InstructionContext {
            instruction,
            arguments: [
                self.read_at_position(self.read_at_position(self.ip + 1)? as usize)?,
                self.read_at_position(self.read_at_position(self.ip + 2)? as usize)?,
            ],
            result_position: self.ram[self.ip + 3] as usize,
        };

        self.ip += 4;

        Ok(result)
    }

    fn read_at_position(&self, position: usize) -> Result<u64, MachineError> {
        if self.ram.len() <= position {
            return Err(MachineError::OutOfBound(position));
        }

        Ok(self.ram[position])
    }

    fn write_at_position(&mut self, position: usize, value: u64) -> Result<(), MachineError> {
        if self.ram.len() <= position {
            return Err(MachineError::OutOfBound(position));
        }

        self.ram[position] = value;
        Ok(())
    }

    pub fn run(mut self) -> Result<Vec<u64>, MachineError> {
        loop {
            let instruction_ctx = self.read_instruction();

            if let Err(MachineError::Exiting) = instruction_ctx {
                break;
            }

            let instruction_ctx = instruction_ctx?;

            match instruction_ctx.instruction {
                InstructionType::Addition => self.write_at_position(
                    instruction_ctx.result_position,
                    instruction_ctx.arguments[0] + instruction_ctx.arguments[1],
                )?,
                InstructionType::Multiplication => self.write_at_position(
                    instruction_ctx.result_position,
                    instruction_ctx.arguments[0] * instruction_ctx.arguments[1],
                )?,
                InstructionType::Exit => break,
            }
        }

        Ok(self.ram)
    }
}

fn read_code(input_file: &str) -> std::io::Result<Vec<u64>> {
    let reader = BufReader::new(File::open(input_file)?);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let opcodes = line.split(',');

        for opcode in opcodes {
            let opcode_value =
                u64::from_str_radix(opcode, 10).expect("Cannot parse a line as a valid u64");
            result.push(opcode_value);
        }
    }

    Ok(result)
}

fn main() -> std::io::Result<()> {
    let part = env::args().nth(1).expect("Please a part (1 or 2)");
    let input_path = env::args()
        .nth(2)
        .expect("Please provide a file as argument");

    let code = read_code(&input_path)?;

    match part.as_str() {
        "1" => {
            let machine = IntMachine::new(code);
            let result = machine
                .run()
                .expect("Exception during machine execution")
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            println!("{}", result.join(","));
        }
        "2" => {
            for noun in 0..99 {
                for verb in 0..99 {
                    let mut custom_code = code.clone();

                    custom_code[1] = noun;
                    custom_code[2] = verb;

                    let machine = IntMachine::new(custom_code);

                    let ram = machine.run().expect("No error here");

                    if ram[0] == 19_690_720 {
                        println!("Found result: nom: {}, verb: {}", noun, verb);
                        println!("100 * noun + verb = {}", 100 * noun + verb);

                        std::process::exit(0);
                    }
                }
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    fn run_machine(code: Vec<u64>, expected_result: Vec<u64>) {
        use super::IntMachine;
        let machine = IntMachine::new(code);

        assert_eq!(machine.run(), Ok(expected_result));
    }

    #[test]
    pub fn test_instructions() {
        run_machine(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        run_machine(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        run_machine(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        run_machine(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
