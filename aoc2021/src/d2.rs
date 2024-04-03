use std::{
  fs::File,
  io::{self, BufRead},
};

enum Instruction {
  Forward(usize),
  Down(usize),
  Up(usize),
}

fn read_input() -> Vec<Instruction> {
  io::BufReader::new(File::open("day2.txt").unwrap())
    .lines()
    .map(
      |line| match line.unwrap().split(' ').collect::<Vec<&str>>()[..] {
        ["forward", n] => Instruction::Forward(n.parse::<usize>().unwrap()),
        ["up", n] => Instruction::Up(n.parse::<usize>().unwrap()),
        ["down", n] => Instruction::Down(n.parse::<usize>().unwrap()),
        _ => panic!(),
      },
    )
    .collect()
}

pub fn d2e1() {
  let result = read_input().into_iter().fold((0, 0), |acc, e| match e {
    Instruction::Forward(n) => (acc.0 + n, acc.1),
    Instruction::Up(n) => (acc.0, acc.1 - n),
    Instruction::Down(n) => (acc.0, acc.1 + n),
  });
  let result = result.0 * result.1;

  println!("{result:#?}");
}

pub fn d2e2() {
  let result = read_input().into_iter().fold((0, 0, 0), |acc, e| match e {
    Instruction::Forward(n) => (acc.0 + n, acc.1 + n * acc.2, acc.2),
    Instruction::Up(n) => (acc.0, acc.1, acc.2 - n),
    Instruction::Down(n) => (acc.0, acc.1, acc.2 + n),
  });
  let result = result.0 * result.1;

  println!("{result:#?}");
}
