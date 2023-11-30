#![allow(non_snake_case)]

use std::process;
use Y2023::utils::file_reader::get_task_and_data;
fn main() {
  let meta_data = get_task_and_data();

  let meta = match meta_data {
    Ok(m) => m,
    Err(e) => {
      println!("{}", e.get_reason());
      process::exit(1);
    }
  };

  dbg!(meta);
}
