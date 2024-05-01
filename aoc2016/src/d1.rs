use std::{fs::File, io::Read};

enum Direction {
  Left,
  Right,
}

#[derive(Clone, Copy, Default)]
enum Facing {
  #[default]
  North,
  West,
  East,
  South,
}

#[derive(Default)]
struct State {
  facing: Facing,
  north_south: isize,
  west_east: isize,
}

impl Facing {
  fn turn(self, dir: Direction) -> Self {
    match dir {
      Direction::Left => match self {
        Self::North => Self::West,
        Self::West => Self::South,
        Self::East => Self::North,
        Self::South => Self::East,
      },
      Direction::Right => match self {
        Self::North => Self::East,
        Self::West => Self::North,
        Self::East => Self::South,
        Self::South => Self::West,
      }
    }
  }
}

impl State {
  fn advance(&mut self, amount: usize) {
    match self.facing {
      Facing::North => {
        self.north_south += amount as isize
      },
      Facing::West => {
        self.west_east += amount as isize
      },
      Facing::East => {
        self.west_east -= amount as isize
      },
      Facing::South => {
        self.north_south -= amount as isize
      },
    }
  }

  fn turn(&mut self, dir: Direction) {
    self.facing = self.facing.turn(dir);
  }

  fn distance(&self) -> isize {
    self.north_south.abs() + self.west_east.abs()
  }
}

pub fn d1e1() {
  let instructions = read_instructions();
  let mut state: State = State::default();

  for (dir, number) in instructions {
    state.turn(dir);
    state.advance(number);
  }

  println!("{:#?}", &state.distance())
}

fn read_instructions() -> Vec<(Direction, usize)> {
    let filename = "day1.txt";
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.split(',')
        .map(|s| {
          let (dir_char, rest) = s.trim().split_at(1);
          let direction = match dir_char {
            "L" => Direction::Left,
            "R" => Direction::Right,
            a => {
              println!("{a:?}");
              panic!()
            }
          };
          let number = rest.parse::<usize>().unwrap();
          (direction, number)
        }).collect()
}
