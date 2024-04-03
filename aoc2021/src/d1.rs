use std::{
  fs::File,
  io::{self, BufRead},
};

pub fn d1e1() {
  let result = count_increases(read_input());

  println!("{result:#?}");
}

pub fn d1e2() {
  let result = count_increases(group_by_3(read_input()));

  println!("{result:#?}");
}

fn read_input() -> Vec<usize> {
  io::BufReader::new(File::open("day1.txt").unwrap())
    .lines()
    .map(|line| line.unwrap().parse::<usize>().unwrap())
    .collect()
}

fn count_increases(numbers: Vec<usize>) -> usize {
  numbers
    .windows(2)
    .filter(|window| window[0] < window[1])
    .count()
}

fn group_by_3(numbers: Vec<usize>) -> Vec<usize> {
  numbers
    .windows(3)
    .map(|window| window.iter().sum())
    .collect()
}
