#![allow(non_snake_case)]
#![allow(unreachable_code)]

use std::process;
use Y2023::utils::{file_reader::{get_task_and_data, Meta}, errors::Fault};
use Y2023::tasks::day1;
fn main() {
  let meta_data: Result<Meta, Fault> = get_task_and_data();

  let meta: Meta = match meta_data {
    Ok(m) => m,
    Err(e) => {
      println!("{}", e.get_reason());
      process::exit(1);
      panic!();
    }
  };

  let result = match meta.task.as_str() {
    "1" => day1::day_1(meta.clone()),
    _ => panic!("Given Task number is not available")
  };

  println!("Done {:?}", result);
}
