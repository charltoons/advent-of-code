#[derive(Debug)]
pub enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

pub fn get_instructions(input: &String) -> Vec<Instruction> {
  let mut instructions = Vec::new();
  for instruction_str in input.split("\n").into_iter() {
      let kind = match instruction_str.split(" ").nth(0) {
          Some(kind) => kind,
          None => continue,
      };
      let value = match instruction_str.split(" ").nth(1) {
          Some(value) => match value.parse::<usize>() {
              Ok(value) => value,
              Err(_) => continue,
          },
          None => continue,
      };

      let instruction = match kind {
          "forward" => Instruction::Forward(value),
          "down" => Instruction::Down(value),
          "up" => Instruction::Up(value),
          _ => panic!("Unknown instruction kind: {}", kind),
      };

      instructions.push(instruction);
  }
  instructions
}

pub fn part_1(instructions: &Vec<Instruction>) -> usize {
  let mut position = 0;
  let mut depth = 0;

  for instruction in instructions.iter() {
      match instruction {
          Instruction::Forward(value) => {
              position += value;
          },
          Instruction::Down(value) => {
              depth += value;
          },
          Instruction::Up(value) => {
              depth -= value;
          },
      }
  }

  position * depth
}

pub fn part_2(instructions: &Vec<Instruction>) -> usize {
  let mut position = 0;
  let mut depth = 0;
  let mut aim = 0;

  for instruction in instructions.iter() {
      match instruction {
          Instruction::Forward(value) => {
              position += value;
              depth += aim * value;
          },
          Instruction::Down(value) => {
              aim += value;
          },
          Instruction::Up(value) => {
              aim -= value;
          },
      }
  }

  position * depth
}