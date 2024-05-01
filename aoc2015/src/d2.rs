use std::{
  cmp::max,
  fs::File,
  io::{self, BufRead, Read},
};

fn read_input() -> Vec<(usize, usize, usize)> {
  io::BufReader::new(File::open("day2.txt").unwrap())
    .lines()
    .map(
      |line| match line.unwrap().split('x').collect::<Vec<&str>>()[..] {
        [w, d, h] => (
          w.parse::<usize>().unwrap(),
          d.parse::<usize>().unwrap(),
          h.parse::<usize>().unwrap(),
        ),
        _ => panic!(),
      },
    )
    .collect()
}

pub fn d2e1() {
  let result: usize = read_input()
    .into_iter()
    .map(|(a, b, c)| 2 * a * b + 2 * a * c + 2 * b * c + (a * b * c / max(max(a, b), c)))
    .sum();

  println!("{result}")
}

pub fn d2e2() {
  let result: usize = read_input()
    .into_iter()
    .map(|(a, b, c)| a * b * c + ((a + b + c - max(a, max(b, c))) * 2))
    .sum();

  println!("{result}")
}
