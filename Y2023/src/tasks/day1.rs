use crate::utils::{file_reader::Meta, errors::Fault};
use std::str::FromStr;

// twone is possible -> 21
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

fn filter_digits_in_string(string: &str) -> Vec<char> {
  string.chars()
  .filter(|character| character.is_ascii_digit())
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

fn callibration_calculation_part_1 (data: Vec<&str>) -> Result<i32, Fault> {
  let mut solution_part_1: Vec<i32> = vec![];

  for part in data {
    let dirty = filter_digits_in_string(part);
    let dirty_number = generate_i32_from_vec_char_digits(dirty)?;
    solution_part_1.push(dirty_number);
  }
  Ok(solution_part_1.into_iter().reduce(|a, b| a + b).unwrap())
}

fn callibration_calculation_part_2 (data: Vec<&str>) -> Result<i32, Fault> {
  let mut solution_part_2: Vec<i32> = vec![];

  for part in data {
    let cleaned = replace_str_with_digits(part);
    let numbers_only: Vec<char> = filter_digits_in_string(cleaned.as_str());
    let number = generate_i32_from_vec_char_digits(numbers_only)?;
    solution_part_2.push(number);
  }
  Ok(solution_part_2.into_iter().reduce(|a, b| a + b).unwrap())
}

pub fn day_1 (meta: Meta) -> Result<i32, Fault> {
  let lines: Vec<&str> = meta.get_lines_from_file();

  println!("The sum of all calibration values is {}", callibration_calculation_part_1(lines.clone()).unwrap());
  println!("The sum of all correct calibration values is {}", callibration_calculation_part_2(lines).unwrap());
  Ok(0)
}

#[cfg(test)]
mod tests {
  use crate::{tasks::day1::{replace_str_with_digits, filter_digits_in_string, callibration_calculation_part_1, callibration_calculation_part_2}, utils::file_reader::Meta};

  #[test]
  fn twone_results_21 () {
    assert_eq!(replace_str_with_digits("twone"), "t2o1e")
  }

  #[test]
  fn only_get_digits_from_string () {
    assert_eq!(filter_digits_in_string("abcd4gnu32"), vec!['4', '3', '2'])
  }

  #[test]
  fn full_test_part_1 () {
    let meta = Meta { task: "1".to_string(), file: "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string() };
    let lines = meta.get_lines_from_file();
    assert_eq!(callibration_calculation_part_1(lines).unwrap(), 142);
  }
  
  #[test]
  fn full_test_part_2 () { 
    let meta = Meta { task: "1".to_string(), file: "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string() };
    let lines = meta.get_lines_from_file();
    assert_eq!(callibration_calculation_part_2(lines).unwrap(), 281);
  }
}