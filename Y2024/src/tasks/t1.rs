use std::iter::zip;

use crate::utils::string::split_by_space_to_tuple;

use super::TaskData;

fn convert_lines_into_usize_tuple(lines: Vec<&str>) -> (Vec<usize>, Vec<usize>) {
  lines
  .into_iter()
  .map(|datum| split_by_space_to_tuple(datum))
  .map(|(first, second)| (first.parse::<usize>().unwrap(),
 second.parse::<usize>().unwrap()))
  .unzip()
}

fn part1(left: &Vec<usize>, right: &Vec<usize>) -> Result<String, String> {
  let zipped: Option<usize> = zip(left, right)
    .map(|(first, second)| first.abs_diff(*second))
    .reduce(|acc, d| acc + d);

  match zipped {
    Some(result) => Ok(format!("Location ID Total Distance is {}", result)),

    None => Err("Unable to zip, map and reduce values".to_string())
  }
}

fn part2(left: Vec<usize>, right: Vec<usize>) -> Result<String, String> {
  // let mut dbg = String::new();
  let mut result: usize = 0;
  let right_iter = right.into_iter();
  for l in left {
    let count = right_iter.clone().filter(|v| *v == l).count();
    // dbg.push_str(format!("l({}) * count({}) = {}\n", l, count, l * count).as_str());
    result += count * l;
  }

  Ok(format!("Similarity Score is {}", result))
}

pub fn execute_day(input: String) -> TaskData
{
  let mut results = TaskData::default();
  let lines: Vec<&str> = input.split('\n').collect();
  
  let (mut left,mut right) = convert_lines_into_usize_tuple(lines);
  left.sort();
  right.sort();
  results.mark_step(String::from("Formatting Done"));

  let res1 = part1(&left, &right);
  results.set_task_1(res1.unwrap());
  results.mark_step(String::from("Task 1 - Completed"));

  let res2 = part2(left, right);
  results.set_task_2(res2.unwrap());
  results.mark_step(String::from("Task 2 - Completed"));

  results
}