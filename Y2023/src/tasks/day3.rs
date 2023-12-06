use crate::utils::{file_reader::Meta, errors::Fault, position::{Vec2D, Movable2D}};

const TEST: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

impl Vec2D {
  pub fn next_row(&mut self) {
    self.x = 0;
    self.y += 1;
  }
}

#[derive(Debug, Copy, Clone)]
struct Number {
  value: u32,
  start: Vec2D,
  length: u8
}

impl Number {
  pub fn new (value: char, coordinates: Vec2D) -> Self {
    Number {
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

  pub fn has_adjacent_symbol (&mut self, symbols: &Vec<Vec2D>) -> bool {
    let mut current = self.start;

    current.move_up();
    current.move_left();
    for _ in 0..=self.length {
      match symbols.into_iter().find(|&&s| s == current) {
        Some(_) => return true,
        _ => {}
      }
      current.move_right();
    }
    match symbols.into_iter().find(|&&s| s == current) {
      Some(_) => return true,
      _ => {}
    }
    current.move_down();
    match symbols.into_iter().find(|&&s| s == current) {
      Some(_) => return true,
      _ => {}
    }
    current.move_down();
    match symbols.into_iter().find(|&&s| s == current) {
      Some(_) => return true,
      _ => {}
    }
    for _ in 0..=self.length {
      match symbols.into_iter().find(|&&s| s == current) {
        Some(_) => return true,
        _ => {}
      }
      current.move_left();
    }
    match symbols.into_iter().find(|&&s| s == current) {
      Some(_) => return true,
      _ => {}
    }
    current.move_up();
    match symbols.into_iter().find(|&&s| s == current) {
      Some(_) => return true,
      _ => {}
    }
    false
  }
}

pub fn day_3 (_meta: Meta) -> Result<i32, Fault> {
  // let splits: Vec<&str> = TEST.split('\n').into_iter().collect();
  let splits = _meta.get_lines_from_file();
  let mut numbers: Vec<Number> = vec![];
  let mut symbols: Vec<Vec2D> = vec![];
  let mut coordinates = Vec2D::zero();

  for y in 0..splits.len() {
    let line = splits[y].chars();
    let _x: u32 = 0;
    let mut current_number: Option<Number> = None;
    for c in line {
      match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          current_number = match current_number {
            Some(mut current) => Some(current.extend(c)),
            None => Some(Number::new(c, coordinates))
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

  let filtered: Vec<Number> = numbers.into_iter().filter(|&n| n.to_owned().has_adjacent_symbol(&symbols)).collect();

  let sum = filtered.into_iter().map(|x| x.value).reduce(|a, b| a+b);
  print!("[DAY 3][PART 1] {:?}", sum);
  Ok(0)
}


#[cfg(test)]
mod tests {

}