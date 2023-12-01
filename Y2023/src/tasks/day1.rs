use crate::utils::{file_reader::Meta, errors::Fault};
use std::str::FromStr;

fn is_char_number(character: &char) -> bool {
  match character {
    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
    _ => false
  }
}

fn replace_str_with_digits(string: &str) -> String {
  string
  .replace("one", "o1e")
  .replace("two", "t2o")
  .replace("three", "t3e")
  .replace("four", "f4r")
  .replace("five", "f5e")
  .replace("six", "s6x")
  .replace("seven", "s7n")
  .replace("eight", "e8t")
  .replace("nine", "n9e")
  .replace("zero", "z0o")
}

fn replace_string_numbers_with_digits(string: &str) -> Vec<char> {
  string.chars()
  .filter(|character| is_char_number(character))
  .collect::<Vec<char>>()
}

fn generate_i32_from_vec_char_digits(chars: Vec<char>) -> Result<i32, Fault> {
  let first_and_last = match (chars.first(), chars.last()) {
    (Some(first), Some(last)) => Ok(format!("{}{}", first, last)),
    _ => Err(Fault::FailedToExtractDigits),
  };
  match first_and_last {
    Ok(result) => i32::from_str(result.as_str()).or_else(|_| Err(Fault::FailedToExtractDigits)),
    Err(x) => Err(x),
  }
}

pub fn day_1 (meta: Meta) -> Result<i32, Fault> {
  let lines: Vec<&str> = meta.get_lines_from_file();
  let mut solution_part_1: Vec<i32> = vec![];
  let mut solution_part_2: Vec<i32> = vec![];

  for part in lines {
    let dirty = replace_string_numbers_with_digits(part);
    let dirty_number = generate_i32_from_vec_char_digits(dirty)?;
    solution_part_1.push(dirty_number);

    let cleaned = replace_str_with_digits(part);
    let numbers_only: Vec<char> = replace_string_numbers_with_digits(cleaned.as_str());
    let number = generate_i32_from_vec_char_digits(numbers_only)?;
    solution_part_2.push(number);
  }

  let result = solution_part_1.into_iter().reduce(|a, b| a + b).unwrap();
  println!("The sum of all calibration values is {}", result);
  let result = solution_part_2.into_iter().reduce(|a, b| a + b).unwrap();
  println!("The sum of all correct calibration values is {}", result);
  Ok(0)
}