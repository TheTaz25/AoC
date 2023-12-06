use crate::utils::{file_reader::Meta, errors::Fault, position::{Vec2D, Movable2D, Collidable2D}};

const TEST: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

impl Vec2D {
  pub fn next_row(&mut self) {
    self.x = 0;
    self.y += 1;
  }
}

#[derive(Debug, Copy, Clone)]
struct PartNumber {
  value: u32,
  start: Vec2D,
  length: u8
}

impl PartNumber {
  pub fn new (value: char, coordinates: Vec2D) -> Self {
    PartNumber {
      value: value.to_digit(10).unwrap(),
      start: coordinates,
      length: 1
    }
  }

  pub fn extend (&mut self, value: char) -> Self {
    let next_value = (self.value * 10) + value.to_digit(10).unwrap();
    self.length += 1;
    self.value = next_value;
    *self
  }
}

impl Collidable2D for PartNumber {
  fn get_collisions (&self) -> Vec<Vec2D> {
    let mut collision_points: Vec<Vec2D> = vec![];
    let mut current = self.start.clone();

    current.move_up();
    current.move_left();

    collision_points.push(current);
    collision_points.push(Vec2D::from_x_y(current.x, current.y + 1));
    collision_points.push(Vec2D::from_x_y(current.x, current.y + 2));
    collision_points.push(Vec2D::from_x_y(self.start.x + Into::<i16>::into(self.length), self.start.y));

    for _ in 0..self.length + 1 {
      current.move_right();
      collision_points.push(current);
      collision_points.push(Vec2D::from_x_y(current.x, current.y + 2));
    }

    collision_points
  }
}


#[derive(Copy, Clone, Debug)]
enum SymbolType {
  STAR, PLUS, EQ, HASH, PERCENT, MINUS, AT, SLASH, AMPERSAND, DOLLAR
}

impl SymbolType {
  pub fn get_symbol (sym: char) -> Self {
    match sym {
      '*' => Self::STAR,
      '+' => Self::PLUS,
      '=' => Self::EQ,
      '#' => Self::HASH,
      '%' => Self::PERCENT,
      '-' => Self::MINUS,
      '@' => Self::AT,
      '/' => Self::SLASH,
      '&' => Self::AMPERSAND,
      '$' => Self::DOLLAR,
      _ => panic!("Unknown symbol {sym}")
    }
  }
}

#[derive(Debug, Copy, Clone)]
struct Symbol {
  sym_type: SymbolType,
  position: Vec2D,
}

impl Collidable2D for Symbol {
  fn get_collisions (&self) -> Vec<Vec2D> {
    vec![
      Vec2D::from_x_y(self.position.x - 1, self.position.y - 1),
      Vec2D::from_x_y(self.position.x - 1, self.position.y),
      Vec2D::from_x_y(self.position.x - 1, self.position.y + 1),
      Vec2D::from_x_y(self.position.x, self.position.y + 1),
      Vec2D::from_x_y(self.position.x, self.position.y - 1),
      Vec2D::from_x_y(self.position.x + 1, self.position.y - 1),
      Vec2D::from_x_y(self.position.x + 1, self.position.y),
      Vec2D::from_x_y(self.position.x + 1, self.position.y + 1),
    ]
  }
}

impl Symbol {
  pub fn new(sym: char, position: Vec2D) -> Self {
    Self { sym_type: SymbolType::get_symbol(sym), position }
  }
}

pub fn day_3 (_meta: Meta) -> Result<i32, Fault> {
  // let splits: Vec<&str> = TEST.split('\n').into_iter().collect();
  let splits = _meta.get_lines_from_file();
  let mut numbers: Vec<PartNumber> = vec![];
  let mut symbols: Vec<Vec2D> = vec![];
  let mut coordinates = Vec2D::zero();

  for y in 0..splits.len() {
    let line = splits[y].chars();
    let _x: u32 = 0;
    let mut current_number: Option<PartNumber> = None;
    for c in line {
      match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          current_number = match current_number {
            Some(mut current) => Some(current.extend(c)),
            None => Some(PartNumber::new(c, coordinates))
          }
        },
        '.' => {
          match current_number {
            Some(current) => {
              numbers.push(current);
              current_number = None;
            },
            _ => {}
          }
        },
        _ => {
          symbols.push(coordinates);
          match current_number {
            Some(current) => {
              numbers.push(current);
              current_number = None;
            }
            _ => {}
          }
        }
      }
      coordinates.move_right();
    }
    match current_number {
      Some(current) => {
        numbers.push(current);
      }
      _ => {}
    }
    coordinates.next_row();
  }

  // let filtered: Vec<PartNumber> = numbers.into_iter().filter(|&n| n.to_owned().has_adjacent_symbol(&symbols)).collect();

  // let sum = filtered.into_iter().map(|x| x.value).reduce(|a, b| a+b);
  // print!("[DAY 3][PART 1] {:?}", sum);
  Ok(0)
}


#[cfg(test)]
mod tests {
    use crate::utils::position::{Vec2D, Collidable2D};

    use super::{PartNumber, Symbol};

  #[test]
  fn part_number_has_collisions() {
    let part = PartNumber { length: 2, value: 22, start: Vec2D::from_x_y(1, 1) };
    let collisions = part.get_collisions();

    assert!(collisions.contains(&Vec2D::from_x_y(0, 0)));
    assert!(collisions.contains(&Vec2D::from_x_y(1, 0)));
    assert!(collisions.contains(&Vec2D::from_x_y(2, 0)));
    assert!(collisions.contains(&Vec2D::from_x_y(3, 0)));
    assert!(collisions.contains(&Vec2D::from_x_y(0, 1)));
    assert!(collisions.contains(&Vec2D::from_x_y(3, 1)));
    assert!(collisions.contains(&Vec2D::from_x_y(0, 2)));
    assert!(collisions.contains(&Vec2D::from_x_y(1, 2)));
    assert!(collisions.contains(&Vec2D::from_x_y(2, 2)));
    assert!(collisions.contains(&Vec2D::from_x_y(3, 2)));
  }

  #[test]
  fn symbol_has_collisions() {
    let symbol = Symbol::new('*', Vec2D::from_x_y(0, 0));
    let collisions = symbol.get_collisions();

    assert!(collisions.contains(&Vec2D::from_x_y(-1, -1)));
    assert!(collisions.contains(&Vec2D::from_x_y(-1, 0)));
    assert!(collisions.contains(&Vec2D::from_x_y(-1, 1)));
    assert!(collisions.contains(&Vec2D::from_x_y(0, -1)));
    assert!(collisions.contains(&Vec2D::from_x_y(0, 1)));
    assert!(collisions.contains(&Vec2D::from_x_y(1, -1)));
    assert!(collisions.contains(&Vec2D::from_x_y(1, 0)));
    assert!(collisions.contains(&Vec2D::from_x_y(1, 1)));
  }
}