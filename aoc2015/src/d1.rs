use std::{fs::File, io::Read};

fn read_input() -> String {
  let filename = "day1.txt";
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

pub fn d1e1() {
  let contents = read_input();

  let result = contents.chars().fold(0isize, |acc, c| match c {
    '(' => acc + 1,
    ')' => acc - 1,
    _ => acc,
  });

  println!("{result:#?}");
}

pub fn d1e2() {
  let contents = read_input();
  let mut floor = 0;

  for (i, el) in contents.chars().enumerate() {
    match el {
      '(' => floor += 1,
      ')' => floor -= 1,
      _ => (),
    }

    if floor == -1 {
      println!("{:#?}", i + 1);
      return;
    }
  }
}
