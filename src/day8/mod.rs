use std::str::FromStr;
use crate::utils::GenericError;
use crate::daily_challenge::DailyChallenge;

#[derive(Debug, PartialEq, Clone)]
pub enum RunCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: RunCode,
    pub param: isize,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub accumulator: isize,
    pub current_line: usize,
    pub line_execution_count: Vec<usize>,
}

impl Program {
    pub fn execute_once(&mut self) -> Result<(), GenericError> {
        if self.current_line >= self.instructions.len() {
            return GenericError::throw("Out of range");
        }

        self.line_execution_count[self.current_line] += 1;

        let current_instruction = &self.instructions[self.current_line];
        match current_instruction.name {
            RunCode::Acc => {
                self.accumulator += current_instruction.param;
                self.current_line += 1;
            }
            RunCode::Jmp => {
                if current_instruction.param.is_positive() {
                    self.current_line += current_instruction.param.abs() as usize;
                } else {
                    self.current_line -= current_instruction.param.abs() as usize;
                }
            }
            RunCode::Nop => self.current_line += 1
        }

        Ok(())
    }

    pub fn execute_until_loop(&mut self, max_loop_count: usize) -> Result<(), GenericError> {
        let mut will_reach_limit = self.will_reach_limit(max_loop_count);
        while !will_reach_limit {
            self.execute_once()?;
            will_reach_limit = self.will_reach_limit(max_loop_count);
        }

        Ok(())
    }

    fn will_reach_limit(&self, max_loop_count: usize) -> bool {
        self.line_execution_count[self.current_line] >= (max_loop_count - 1)
    }
}

impl FromStr for Instruction {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() < 2 {
            return GenericError::throw("No space in string");
        }

        let name = parts[0].parse()?;
        let param: isize = parts[1].parse()?;

        Ok(Instruction {
            name,
            param,
        })
    }
}

impl FromStr for RunCode {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "nop" {
            Ok(RunCode::Nop)
        } else if s == "acc" {
            Ok(RunCode::Acc)
        } else if s == "jmp" {
            Ok(RunCode::Jmp)
        } else {
            GenericError::throw("Invalid instruction")
        }
    }
}

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Self {
        let line_count = instructions.len();
        Program {
            instructions,
            current_line: 0,
            accumulator: 0,
            line_execution_count: vec![0; line_count],
        }
    }
}

#[derive(Default)]
pub struct Day8 ();

impl DailyChallenge for Day8 {
    type Data = Instruction;
    type Wrapper = Program;

    fn get_day_num(&self) -> usize { 8 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut program = data.clone();
        program.execute_until_loop(2)?;
        Ok(format!("Accumulator value before first loop is {}", program.accumulator))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        Ok(String::from("unimplemented"))
    }
}

mod tests {
    use crate::day8::{Instruction, RunCode, Program};
    use std::str::FromStr;

    #[test]
    pub fn it_can_parse_input() {
        let input = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6"
        ];

        let instructions_result: Result<Vec<Instruction>, _> = input.into_iter()
            .map(|s| Instruction::from_str(s))
            .collect();

        assert!(instructions_result.is_ok());

        let instructions = instructions_result.unwrap();
        assert_eq!(instructions.len(), 9);
        assert_eq!(instructions[5].name, RunCode::Acc);
        assert_eq!(instructions[5].param, -99);

        let mut program: Program = instructions.into();
        assert_eq!(program.instructions.len(), 9);

        assert!(program.execute_once().is_ok());
        assert_eq!(program.current_line, 1);
        assert_eq!(program.accumulator, 0);

        assert!(program.execute_once().is_ok());
        assert_eq!(program.current_line, 2);
        assert_eq!(program.accumulator, 1);

        assert!(program.execute_once().is_ok());
        assert_eq!(program.current_line, 6);
        assert_eq!(program.accumulator, 1);

        assert!(program.execute_until_loop(2).is_ok());
        assert_eq!(program.accumulator, 5);
    }
}
