use std::collections::HashMap;
use std::fs::read_to_string;

enum ParseResult {
  Yay(usize, String),
  Nope(String),
}

fn main() {
  let lines: Vec<String> = read_lines("./input");
  let pairs: usize = lines
    .iter()
    .map(|line| {
      let (first, last) = parse_recurse(line, vec![]);
      10 * first + last
    })
    .sum();
  println!("{:?}", pairs)
}

fn parse(graphemes: &String) -> ParseResult {
  let prefixes: Vec<&str> = vec![
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
  ];
  let map: HashMap<&str, i32> = HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
  ]);

  let matching_prefix: Option<&&str> = prefixes
    .iter()
    .find(|&&prefix| graphemes.starts_with(prefix));

  match matching_prefix {
    Some(prefix) => {
      let (first, second) = graphemes.split_at(prefix.len());
      let value: usize = *map.get(first).unwrap() as usize;
      ParseResult::Yay(value, graphemes[1..].to_string())
    }
    None => ParseResult::Nope(graphemes[1..].to_string()),
  }
}

fn parse_recurse(graphemes: &String, tokens: Vec<usize>) -> (usize, usize) {
  println!("{:?}", graphemes);
  if graphemes.len() == 0 {
    (*tokens.first().unwrap(), *tokens.last().unwrap())
  } else {
    let parse_result = parse(graphemes);
    match parse_result {
      ParseResult::Nope(graphemes_tail) => parse_recurse(&graphemes_tail, tokens),
      ParseResult::Yay(num, graphemes_tail) => {
        let new_tokens: Vec<usize> = tokens.iter().cloned().chain(std::iter::once(num)).collect();
        parse_recurse(&graphemes_tail, new_tokens)
      }
    }
  }
}

fn read_lines(filename: &str) -> Vec<String> {
  read_to_string(filename)
    .unwrap() // panic on possible file-reading errors
    .lines() // split the string into an iterator of string slices
    .map(String::from) // make each slice into a string
    .collect() // gather them together into a vector
}
