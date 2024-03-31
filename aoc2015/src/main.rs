use std::{fs::File, io::Read};

fn main() {
  let filename = "day1.txt";
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let result = contents.chars().fold(0isize, |acc, c| match c {
    '(' => acc + 1,
    ')' => acc - 1,
    _ => acc,
  });
  println!("{result:#?}");

  if let Some(result) = fun_name(contents) {
    println!("{result:#?}");
  }
}

fn fun_name(contents: String) -> Option<usize> {
  let mut floor = 0;

  for (i, el) in contents.chars().enumerate() {
    match el {
      '(' => floor += 1,
      ')' => floor -= 1,
      _ => (),
    }

    if floor == -1 {
      return Some(i + 1);
    }
  };
  None
}
