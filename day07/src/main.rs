use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type IntMachineType = i64;

struct IntMachine {
    ip: usize,
    ram: Vec<IntMachineType>,
    output_values: Vec<IntMachineType>,
}

#[derive(Debug, PartialEq, Eq)]
enum MachineError {
    Exiting,
    InvalidInstruction(usize),
    OutOfBound(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum MachineReturn {
    Output(IntMachineType),
    Exit(Vec<IntMachineType>),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    pub fn from_opcode(opcode: usize, position: usize) -> Self {
        let raw_mode = (opcode as usize / usize::pow(10, (position + 2) as u32)) % 10;

        match raw_mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct InstructionArgument {
    value: IntMachineType,
    parameter_mode: ParameterMode,
    argument_position: usize,
}

impl InstructionArgument {
    pub fn get_value(&self, machine: &IntMachine) -> Result<IntMachineType, MachineError> {
        match self.parameter_mode {
            ParameterMode::Immediate => Ok(self.value),
            ParameterMode::Position => machine.read_at_position(self.value as usize),
        }
    }

    pub fn write_value(
        &self,
        machine: &mut IntMachine,
        value: IntMachineType,
    ) -> Result<(), MachineError> {
        match self.parameter_mode {
            ParameterMode::Immediate => machine.write_at_position(self.argument_position, value),
            ParameterMode::Position => machine.write_at_position(self.value as usize, value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum InstructionType {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit,
}

impl InstructionType {
    pub fn from_opcode(opcode: usize) -> Result<InstructionType, MachineError> {
        match opcode % 100 {
            1 => Ok(InstructionType::Addition),
            2 => Ok(InstructionType::Multiplication),
            3 => Ok(InstructionType::Input),
            4 => Ok(InstructionType::Output),
            5 => Ok(InstructionType::JumpIfTrue),
            6 => Ok(InstructionType::JumpIfFalse),
            7 => Ok(InstructionType::LessThan),
            8 => Ok(InstructionType::Equals),
            99 => Ok(InstructionType::Exit),
            _ => Err(MachineError::InvalidInstruction(opcode)),
        }
    }

    pub fn arguments_count(self) -> usize {
        match self {
            InstructionType::Addition => 3,
            InstructionType::Multiplication => 3,
            InstructionType::Input => 1,
            InstructionType::Output => 1,
            InstructionType::JumpIfTrue => 2,
            InstructionType::JumpIfFalse => 2,
            InstructionType::LessThan => 3,
            InstructionType::Equals => 3,
            InstructionType::Exit => 0,
        }
    }

    pub fn arguments_configuration(self, opcode: usize) -> Vec<ParameterMode> {
        let mut result = Vec::new();

        for i in 0..self.arguments_count() {
            result.push(ParameterMode::from_opcode(opcode, i))
        }

        result
    }

    pub fn code_size(self) -> usize {
        self.arguments_count() + 1
    }

    pub fn read_instruction(
        self,
        machine: &IntMachine,
    ) -> Result<InstructionContext, MachineError> {
        let mut arguments = Vec::new();

        for (i, parameter_mode) in self
            .arguments_configuration(machine.read_at_position(machine.ip)? as usize)
            .iter()
            .enumerate()
        {
            arguments.push(InstructionArgument {
                value: machine.read_at_position(machine.ip + i + 1)?,
                parameter_mode: *parameter_mode,
                argument_position: machine.ip + i + 1,
            });
        }

        Ok(InstructionContext {
            instruction: self,
            arguments,
        })
    }
}

#[derive(Debug)]
struct InstructionContext {
    instruction: InstructionType,
    arguments: Vec<InstructionArgument>,
}

impl IntMachine {
    pub fn new(ram: Vec<IntMachineType>) -> Self {
        IntMachine {
            ip: 0,
            ram,
            output_values: Vec::new(),
        }
    }

    fn read_instruction(&mut self) -> Result<InstructionContext, MachineError> {
        if self.ram.len() <= self.ip {
            return Err(MachineError::OutOfBound(self.ip));
        }

        let instruction = InstructionType::from_opcode(self.ram[self.ip] as usize)?;

        if instruction == InstructionType::Exit {
            return Err(MachineError::Exiting);
        }

        let result = instruction.read_instruction(self)?;

        self.ip += result.instruction.code_size();

        Ok(result)
    }

    fn read_at_position(&self, position: usize) -> Result<IntMachineType, MachineError> {
        if self.ram.len() <= position {
            return Err(MachineError::OutOfBound(position));
        }

        Ok(self.ram[position])
    }

    fn write_at_position(
        &mut self,
        position: usize,
        value: IntMachineType,
    ) -> Result<(), MachineError> {
        if self.ram.len() <= position {
            return Err(MachineError::OutOfBound(position));
        }

        self.ram[position] = value;
        Ok(())
    }

    pub fn run(
        &mut self,
        ram_dump: bool,
        break_at_output: bool,
        mut input_values: Vec<IntMachineType>,
    ) -> Result<MachineReturn, MachineError> {
        loop {
            let instruction_ctx = self.read_instruction();

            if let Err(MachineError::Exiting) = instruction_ctx {
                break;
            }

            let instruction_ctx = instruction_ctx?;

            match instruction_ctx.instruction {
                InstructionType::Addition => {
                    let value_a = instruction_ctx.arguments[0].get_value(&self)?;
                    let value_b = instruction_ctx.arguments[1].get_value(&self)?;

                    instruction_ctx.arguments[2].write_value(self, value_a + value_b)?;
                }
                InstructionType::Multiplication => {
                    let value_a = instruction_ctx.arguments[0].get_value(&self)?;
                    let value_b = instruction_ctx.arguments[1].get_value(&self)?;

                    instruction_ctx.arguments[2].write_value(self, value_a * value_b)?;
                }
                InstructionType::Input => {
                    let input_value = input_values.remove(0);
                    instruction_ctx.arguments[0].write_value(self, input_value)?;
                }
                InstructionType::Output => {
                    let value = instruction_ctx.arguments[0].get_value(&self)?;

                    self.output_values.push(value);

                    if break_at_output {
                        return Ok(MachineReturn::Output(value));
                    }
                }
                InstructionType::JumpIfTrue => {
                    let value = instruction_ctx.arguments[0].get_value(&self)?;
                    let new_ip = instruction_ctx.arguments[1].get_value(&self)?;

                    if value != 0 {
                        self.ip = new_ip as usize;
                    }
                }
                InstructionType::JumpIfFalse => {
                    let value = instruction_ctx.arguments[0].get_value(&self)?;
                    let new_ip = instruction_ctx.arguments[1].get_value(&self)?;

                    if value == 0 {
                        self.ip = new_ip as usize;
                    }
                }
                InstructionType::LessThan => {
                    let value_a = instruction_ctx.arguments[0].get_value(&self)?;
                    let value_b = instruction_ctx.arguments[1].get_value(&self)?;

                    let result_value = if value_a < value_b { 1 } else { 0 };

                    instruction_ctx.arguments[2].write_value(self, result_value)?;
                }
                InstructionType::Equals => {
                    let value_a = instruction_ctx.arguments[0].get_value(&self)?;
                    let value_b = instruction_ctx.arguments[1].get_value(&self)?;

                    let result_value = if value_a == value_b { 1 } else { 0 };

                    instruction_ctx.arguments[2].write_value(self, result_value)?;
                }
                InstructionType::Exit => break,
            }
        }

        if ram_dump {
            Ok(MachineReturn::Exit(self.ram.clone()))
        } else {
            Ok(MachineReturn::Exit(self.output_values.clone()))
        }
    }
}

fn read_code(input_file: &str) -> std::io::Result<Vec<IntMachineType>> {
    let reader = BufReader::new(File::open(input_file)?);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let opcodes = line.split(',');

        for opcode in opcodes {
            let opcode_value = IntMachineType::from_str_radix(opcode, 10)
                .expect("Cannot parse a line as a valid u64");
            result.push(opcode_value);
        }
    }

    Ok(result)
}

fn run_amplificator_job(
    code: Vec<IntMachineType>,
    amplificator_inputs: Vec<IntMachineType>,
    break_at_output: bool,
) -> Result<IntMachineType, MachineError> {
    let mut signal = 0;

    let mut machines = Vec::new();

    for amplificator_input in amplificator_inputs {
        machines.push((amplificator_input, IntMachine::new(code.clone())));
    }

    if !break_at_output {
        for (amplificator_input, machine) in machines.iter_mut() {
            match machine.run(false, false, vec![*amplificator_input, signal])? {
                MachineReturn::Exit(mut output_values) => {
                    signal = output_values.remove(0);
                }
                MachineReturn::Output(output_value) => signal = output_value,
            };
        }
    } else {
        let mut running = true;
        let mut first_run = true;

        while running {
            for (amplificator_input, machine) in machines.iter_mut() {
                let input_values = if first_run {
                    vec![*amplificator_input, signal]
                } else {
                    vec![signal]
                };

                match machine.run(false, true, input_values)? {
                    MachineReturn::Exit(_) => {
                        running = false;
                        break;
                    }
                    MachineReturn::Output(output_value) => {
                        signal = output_value;
                    }
                };
            }
            first_run = false;
        }
    }

    Ok(signal)
}

fn main() -> std::io::Result<()> {
    let part = env::args().nth(1).expect("Please a part (1 or 2)");

    let input_path = env::args()
        .nth(2)
        .expect("Please provide a file as argument");

    let code = read_code(&input_path)?;

    match part.as_str() {
        "1" => {
            let mut max_signal_opt = None;
            let mut result = None;
            for i in 0..=4 {
                for w in 0..=4 {
                    for x in 0..=4 {
                        for y in 0..=4 {
                            for z in 0..=4 {
                                let signal =
                                    run_amplificator_job(code.clone(), vec![i, w, x, y, z], false)
                                        .expect("Int Machine error");

                                if i == w
                                    || i == x
                                    || i == y
                                    || i == z
                                    || w == x
                                    || w == y
                                    || w == z
                                    || x == y
                                    || x == z
                                    || y == z
                                {
                                    continue;
                                }

                                if let Some(max_signal) = max_signal_opt {
                                    if max_signal < signal {
                                        max_signal_opt = Some(signal);
                                        result = Some((i, w, x, y, z));
                                    }
                                } else {
                                    max_signal_opt = Some(signal);
                                    result = Some((i, w, x, y, z));
                                }
                            }
                        }
                    }
                }
            }

            println!("Max signal: {:?}", max_signal_opt);
            println!("Result: {:?}", result);
        }
        "2" => {
            let mut max_signal_opt = None;
            let mut result = None;
            for i in 5..=9 {
                for w in 5..=9 {
                    for x in 5..=9 {
                        for y in 5..=9 {
                            for z in 5..=9 {
                                let signal =
                                    run_amplificator_job(code.clone(), vec![i, w, x, y, z], true)
                                        .expect("Int Machine error");

                                if i == w
                                    || i == x
                                    || i == y
                                    || i == z
                                    || w == x
                                    || w == y
                                    || w == z
                                    || x == y
                                    || x == z
                                    || y == z
                                {
                                    continue;
                                }

                                if let Some(max_signal) = max_signal_opt {
                                    if max_signal < signal {
                                        max_signal_opt = Some(signal);
                                        result = Some((i, w, x, y, z));
                                    }
                                } else {
                                    max_signal_opt = Some(signal);
                                    result = Some((i, w, x, y, z));
                                }
                            }
                        }
                    }
                }
            }

            println!("Max signal: {:?}", max_signal_opt);
            println!("Result: {:?}", result);
        }
        _ => unimplemented!(),
    }

    let mut input_value = Vec::new();

    for value in env::args().skip(3) {
        input_value
            .push(IntMachineType::from_str_radix(&value, 10).expect("input should be a number"));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::IntMachineType;
    use super::MachineReturn;

    fn run_machine(
        code: Vec<IntMachineType>,
        expected_result: Vec<IntMachineType>,
        input_value: Vec<IntMachineType>,
        ram_dump: bool,
    ) {
        use super::IntMachine;
        let mut machine = IntMachine::new(code);

        assert_eq!(
            machine.run(ram_dump, false, input_value),
            Ok(MachineReturn::Exit(expected_result))
        );
    }

    #[test]
    pub fn test_instructions() {
        run_machine(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99], vec![0], true);
        run_machine(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99], vec![0], true);
        run_machine(
            vec![2, 4, 4, 5, 99, 0],
            vec![2, 4, 4, 5, 99, 9801],
            vec![0],
            true,
        );
        run_machine(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            vec![0],
            true,
        );
        run_machine(
            vec![1002, 4, 3, 4, 33],
            vec![1002, 4, 3, 4, 99],
            vec![0],
            true,
        );
        run_machine(
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![1],
            vec![8],
            false,
        );
        run_machine(
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![0],
            vec![-42],
            false,
        );
        run_machine(
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![1],
            vec![7],
            false,
        );
        run_machine(
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![0],
            vec![8],
            false,
        );
        run_machine(
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![0],
            vec![9],
            false,
        );
        run_machine(
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            vec![1],
            vec![8],
            false,
        );
        run_machine(
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            vec![0],
            vec![-8],
            false,
        );
        run_machine(
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            vec![0],
            vec![8],
            false,
        );
        run_machine(
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            vec![1],
            vec![-8],
            false,
        );
        run_machine(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![1],
            vec![42],
            false,
        );
        run_machine(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
            vec![0],
            false,
        );
        run_machine(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![1],
            vec![42],
            false,
        );
        run_machine(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![0],
            vec![0],
            false,
        );
        run_machine(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![999],
            vec![7],
            false,
        );
        run_machine(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![1000],
            vec![8],
            false,
        );
        run_machine(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![1001],
            vec![9],
            false,
        );
    }

    #[test]
    pub fn test_part1() {
        use super::run_amplificator_job;

        assert_eq!(
            run_amplificator_job(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                vec![4, 3, 2, 1, 0],
                false
            )
            .unwrap(),
            43210
        );
        assert_eq!(
            run_amplificator_job(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                vec![0, 1, 2, 3, 4],
                false
            )
            .unwrap(),
            54321
        );
        assert_eq!(
            run_amplificator_job(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                vec![1, 0, 4, 3, 2],
                false
            )
            .unwrap(),
            65210
        );
    }

    #[test]
    pub fn test_part2() {
        use super::run_amplificator_job;

        assert_eq!(
            run_amplificator_job(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                vec![9, 8, 7, 6, 5],
                true
            )
            .unwrap(),
            139629729
        );
        assert_eq!(
            run_amplificator_job(
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                vec![9, 7, 8, 5, 6],
                true
            )
            .unwrap(),
            18216
        );
    }
}
