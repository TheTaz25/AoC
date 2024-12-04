use std::fmt::Display;

use super::TaskData;

#[derive(Debug, Copy, Clone)]
struct Coordinate (char, isize, isize);

impl Coordinate {
  pub fn new(c: char, x: isize, y: isize) -> Self {
    Self (c, x, y)
  }

  pub fn coordinates(self) -> (isize, isize) {
    (self.1, self.2)
  }
}

impl PartialEq for Coordinate {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
      && self.1 == other.1
      && self.2 == other.2
  }
}

impl Display for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, at {}, {})", self.0, self.1, self.2)
  }
}

fn convert_input_to_map(input: String, results: &mut TaskData) -> Vec<Coordinate> {
  let coordinates: Vec<Coordinate> = (0..).zip(
    input.split('\n').map(|l| (0..).zip(l.chars()).collect())
  ).flat_map(|(y, v): (i32, Vec<(i32, char)>)| {
    v.iter().map(|(x, c)| Coordinate::new(*c, *x as isize, y as isize)).collect::<Vec<Coordinate>>()
  }).collect();

  results.mark_step(format!("Conversion Done"));

  coordinates
}

/// S..S..S
/// .A.A.A.
/// ..MMM..
/// SAMXMAS
/// ..MMM..
/// .A.A.A.
/// S..S..S

const XMAS_DIRECTIONS: [(isize, isize); 8] = [
  (0, -1), // UP
  (1, -1), // UP-RIGHT
  (1, 0), // RIGHT
  (1, 1), // DOWN-RIGHT
  (0, 1), // DOWN
  (-1, 1), // DOWN-LEFT
  (-1, 0), // LEFT
  (-1, -1), // UP-LEFT
];

fn possible_xmas_vectors(x_vector: &Coordinate, _results: &mut TaskData) -> Vec<Coordinate> {
  let (x, y) = x_vector.coordinates();
  let mut coordinates: Vec<Coordinate> = vec![];
  
  for (n_x, n_y) in XMAS_DIRECTIONS {
    for (index, c) in &mut ['M', 'A', 'S'].iter().enumerate() {
      // results.push_log(format!("index {} == {} at {}, {}", index, *c, x + (n_x * (index + 1) as isize), y + (n_y * (index + 1) as isize)));
      coordinates.push(Coordinate::new(*c, x + (n_x * (index + 1) as isize), y + (n_y * (index + 1) as isize)));
    }
  }

  coordinates
}

fn part1(input: &Vec<Coordinate>, results: &mut TaskData) {
  let only_x: Vec<&Coordinate> = input.iter().filter(|c| c.0 == 'X').collect();

  results.push_log(format!("Part 1 - Found {} start (X) vectors", only_x.len()));

  let mut counter: usize = 0;

  for x in only_x {
    let check_coordinates = possible_xmas_vectors(x, results);
    
    for coords in check_coordinates.chunks(3) {
      match coords {
        [coord_m, coord_a, coord_s] => {
          // results.push_log(format!("{}, {}, {}", coord_m, coord_a, coord_s));
          if input.contains(coord_m) && input.contains(coord_a) && input.contains(coord_s) {
            counter += 1;
          }
        },
        _ => {
          results.push_log(format!("Part 1 - Unexpected number of coordinates when checking possible xmas-vectors!"));
        }
      }
    }
  }

  results.set_task_1(format!("Part 1 - Found {} occurences of the word 'XMAS'", counter));
  results.mark_step("Find all XMAS".to_string());
}

fn possible_mas_vectors(a: &Coordinate, _results: &mut TaskData) -> Vec<Coordinate> {
  let (x, y) = a.coordinates();
  let coordinates: Vec<Coordinate> = vec![
    // M.S
    // .A.
    // M.S
    Coordinate::new('M', x - 1, y - 1),
    Coordinate::new('S', x + 1, y - 1),
    Coordinate::new('M', x - 1, y + 1),
    Coordinate::new('S', x + 1, y + 1),

    // M.M
    // .A.
    // S.S
    Coordinate::new('M', x - 1, y - 1),
    Coordinate::new('M', x + 1, y - 1),
    Coordinate::new('S', x - 1, y + 1),
    Coordinate::new('S', x + 1, y + 1),

    // S.M
    // .A.
    // S.M
    Coordinate::new('S', x - 1, y - 1),
    Coordinate::new('M', x + 1, y - 1),
    Coordinate::new('S', x - 1, y + 1),
    Coordinate::new('M', x + 1, y + 1),

    // S.S
    // .A.
    // M.M
    Coordinate::new('S', x - 1, y - 1),
    Coordinate::new('S', x + 1, y - 1),
    Coordinate::new('M', x - 1, y + 1),
    Coordinate::new('M', x + 1, y + 1),
  ];

  coordinates
}

fn part2(input: &Vec<Coordinate>, results: &mut TaskData) {
  let only_a: Vec<&Coordinate> = input.iter().filter(|c| c.0 == 'A').collect();

  results.push_log(format!("Part 2 - Found {} start (A) vectors", only_a.len()));

  let mut counter : usize = 0;

  for a in only_a {
    let check_coordinates = possible_mas_vectors(a, results);
    let patterns = check_coordinates.chunks(4);

    for pattern in patterns {
      match pattern {
        [top_left, top_right, bottom_left, bottom_right] => {
          if input.contains(&top_left) && input.contains(&top_right) && input.contains(&bottom_left) && input.contains(&bottom_right) {
            counter += 1;
            break;
          }
        },
        _ => results.push_log(format!("Part 2 - Unexpected number of coordinates when checking possible X-MAS Vectors!")),
      }
    }
  }

  results.set_task_2(format!("Part 2 - Found {} occurences of the word 'MAS' in X-Pattern", counter));
  results.mark_step("Find all MAS in X Pattern".to_string());
}

pub fn execute_day(input: String) -> TaskData {
  let mut results = TaskData::default();

  let coordinates = convert_input_to_map(input, &mut results);

  part1(&coordinates, &mut results);

  part2(&coordinates, &mut results);

  results
}