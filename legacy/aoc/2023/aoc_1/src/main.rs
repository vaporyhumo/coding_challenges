use std::fs::read_to_string;

fn main() {
  let lines = read_lines("./input");
  let first_chars: Vec<usize> = lines
    .iter()
    .map(|i| {
      i.chars()
        .find(|j| j.is_digit(10))
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
    })
    .collect();
  let last_chars: Vec<usize> = lines
    .iter()
    .map(|i| {
      i.chars()
        .rfind(|j| j.is_digit(10))
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
    })
    .collect();
  let result: usize = first_chars
    .iter()
    .zip(last_chars.iter())
    .map(|(&x, &y)| x * 10 + y)
    .sum();
  println!("{result}");
}

fn read_lines(filename: &str) -> Vec<String> {
  read_to_string(filename)
    .unwrap() // panic on possible file-reading errors
    .lines() // split the string into an iterator of string slices
    .map(String::from) // make each slice into a string
    .collect() // gather them together into a vector
}
