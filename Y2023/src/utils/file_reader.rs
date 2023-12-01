use std::{fs,env};

use super::errors::Fault;

#[derive(Debug, Clone)]
pub struct Meta {
  pub task: String,
  pub file: String,
}

impl Meta {
  pub fn get_lines_from_file (&self) -> Vec<&str> {
    self.file.split('\n').into_iter().collect()
  }
}

pub fn get_task_and_data () -> Result<Meta, Fault> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    return Err(Fault::MissingArguments)
  }

  let input_file: &str = &args[1];
  let input_task: &str = &args[2];

  let file_contents = fs::read_to_string(input_file).or_else(|_| Err(Fault::CannotReadInputFile))?;

  Ok(Meta {
    task: input_task.to_string(),
    file: file_contents,
  })
}