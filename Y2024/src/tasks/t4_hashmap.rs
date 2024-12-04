use std::collections::HashMap;

use super::TaskData;

type Coordinate = HashMap<(isize, isize), char>;
type RawCoordinate = ((isize, isize), char);

fn convert_input_to_map(input: String, results: &mut TaskData) -> HashMap<(isize, isize), char> {
  let coordinates = (0..).zip(
    input.split('\n').map(|l| (0..).zip(l.chars()).collect())
  ).flat_map(|(y, v): (i32, Vec<(i32, char)>)| {
    v.iter().map(|(x, c)| ((*x as isize, y as isize), *c)).collect::<Vec<((isize, isize), char)>>()
  });

  results.mark_step(format!("Conversion Done"));

  HashMap::from_iter(coordinates)
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

fn possible_xmas_vectors(x_vector: (isize, isize) , _results: &mut TaskData) -> Vec<RawCoordinate> {
  let (x, y) = x_vector;
  let mut coordinates: Vec<RawCoordinate> = vec![];
  
  for (n_x, n_y) in XMAS_DIRECTIONS {
    for (index, c) in &mut ['M', 'A', 'S'].iter().enumerate() {
      // results.push_log(format!("index {} == {} at {}, {}", index, *c, x + (n_x * (index + 1) as isize), y + (n_y * (index + 1) as isize)));
      coordinates.push(((x + (n_x * (index + 1) as isize), y + (n_y * (index + 1) as isize)), *c));
    }
  }

  coordinates
}

fn part1(input: &Coordinate, results: &mut TaskData) {
  let only_x: Vec<(isize, isize)> = input.iter().filter(|(_, c)| **c == 'X').map(|i| *i.0).collect();

  results.push_log(format!("Part 1 - Found {} start (X) vectors", only_x.len()));

  let mut counter: usize = 0;

  for x in only_x {
    let check_coordinates = possible_xmas_vectors(x, results);
    
    for coords in check_coordinates.chunks(3) {
      match coords {
        [coord_m, coord_a, coord_s] => {
          // results.push_log(format!("{}, {}, {}", coord_m, coord_a, coord_s));
          if input.get(&coord_m.0).is_some_and(|v| *v == coord_m.1)
          && input.get(&coord_a.0).is_some_and(|v| *v == coord_a.1)
          && input.get(&coord_s.0).is_some_and(|v| *v == coord_s.1) {
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

fn possible_mas_vectors(a: (isize, isize), _results: &mut TaskData) -> Vec<RawCoordinate> {
  let (x, y) = a;
  let coordinates: Vec<RawCoordinate> = vec![
    // M.S
    // .A.
    // M.S
    ((x - 1, y - 1), 'M'),
    ((x + 1, y - 1), 'S'),
    ((x - 1, y + 1), 'M'),
    ((x + 1, y + 1), 'S'),

    // M.M
    // .A.
    // S.S
    ((x - 1, y - 1), 'M'),
    ((x + 1, y - 1), 'M'),
    ((x - 1, y + 1), 'S'),
    ((x + 1, y + 1), 'S'),

    // S.M
    // .A.
    // S.M
    ((x - 1, y - 1), 'S'),
    ((x + 1, y - 1), 'M'),
    ((x - 1, y + 1), 'S'),
    ((x + 1, y + 1), 'M'),

    // S.S
    // .A.
    // M.M
    ((x - 1, y - 1), 'S'),
    ((x + 1, y - 1), 'S'),
    ((x - 1, y + 1), 'M'),
    ((x + 1, y + 1), 'M'),
  ];

  coordinates
}

fn part2(input: &Coordinate, results: &mut TaskData) {
  let only_a: Vec<(isize, isize)> = input.iter().filter(|(_, c)| **c == 'A').map(|(a, _)| *a).collect();

  results.push_log(format!("Part 2 - Found {} start (A) vectors", only_a.len()));

  let mut counter : usize = 0;

  for a in only_a {
    let check_coordinates = possible_mas_vectors(a, results);
    let patterns = check_coordinates.chunks(4);

    for pattern in patterns {
      match pattern {
        [top_left, top_right, bottom_left, bottom_right] => {
          if input.get(&top_left.0).is_some_and(|v| *v == top_left.1)
           && input.get(&top_right.0).is_some_and(|v| *v == top_right.1)
           && input.get(&bottom_left.0).is_some_and(|v| *v == bottom_left.1)
           && input.get(&bottom_right.0).is_some_and(|v| *v == bottom_right.1)
          {
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

// Prefer Hashmap, execution time with hashmaps are mere milliseconds with this size of data
// Using Vec to store this type of data takes multiple seconds for each part
pub fn execute_day(input: String) -> TaskData {
  let mut results = TaskData::default();

  let coordinates = convert_input_to_map(input, &mut results);

  part1(&coordinates, &mut results);

  part2(&coordinates, &mut results);

  results
}