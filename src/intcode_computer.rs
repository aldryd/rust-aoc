use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

// Include these to be able to convert from i32 to the Opcode enum
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

struct Instruction {
    opcode: Opcode,
    lhs: i32,
    rhs: i32,
    destination: i32,
}

pub struct IntcodeComputer {
    program: Vec<i32>,
    instruction_pointer: usize,
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            instruction_pointer: 0,
            program: vec![],
        }
    }

    pub fn run_program(&mut self, program_to_run: Vec<i32>, position_to_return: i32) -> i32 {
        self.program = program_to_run;

        // Run the program until we get the Halt instruction
        loop {
            let the_instruction = self.get_instruction();

            let advance_instruction_pointer: usize;
            match the_instruction.opcode {
                Opcode::Add => advance_instruction_pointer = self.perform_add(the_instruction),
                Opcode::Multiply => advance_instruction_pointer = self.perform_multiply(the_instruction),
                Opcode::Halt => break,
            }

            self.instruction_pointer += advance_instruction_pointer;
        }

        self.program[position_to_return as usize]
    }

    fn get_instruction(&self) -> Instruction {

        const LHS_OFFSET: usize = 1;
        const RHS_OFFSET: usize = 2;
        const DESTINATION_OFFSET: usize = 3;

        let converted_opcode = Opcode::try_from(self.program[self.instruction_pointer]).unwrap_or(Opcode::Add);
        let mut lhs = 0i32;
        let mut rhs = 0i32;
        let mut destination = 0i32;

        if self.instruction_pointer + DESTINATION_OFFSET < self.program.len() && converted_opcode != Opcode::Halt {
            lhs = self.program[self.program[self.instruction_pointer + LHS_OFFSET] as usize];
            rhs = self.program[self.program[self.instruction_pointer + RHS_OFFSET] as usize];
            destination = self.program[self.instruction_pointer + DESTINATION_OFFSET];

            println!(">>>> opcode:{} | lhs:{} (address:{}) | rhs:{} (address:{}) | destination:{}",
                     self.program[self.instruction_pointer], lhs, self.program[self.instruction_pointer + LHS_OFFSET],
                     rhs, self.program[self.instruction_pointer + RHS_OFFSET], destination);
        }

        Instruction { opcode: converted_opcode, lhs, rhs, destination, }
    }

    fn perform_add(&mut self, add_instruction: Instruction) -> usize {
        const ADD_INSTRUCTION_LENGTH: usize = 4;

        let sum = add_instruction.lhs + add_instruction.rhs;
        self.program[add_instruction.destination as usize] = sum;
        return ADD_INSTRUCTION_LENGTH;
    }

    fn perform_multiply(&mut self, multiply_instruction: Instruction) -> usize {
        const MULTIPLY_INSTRUCTION_LENGTH: usize = 4;

        let product = multiply_instruction.lhs * multiply_instruction.rhs;
        self.program[multiply_instruction.destination as usize] = product;
        return MULTIPLY_INSTRUCTION_LENGTH;
    }
}