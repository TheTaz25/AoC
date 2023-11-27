#[derive(Debug)]
pub enum Fault {
  MissingArguments,
  CannotReadInputFile,
}

impl Fault {
  pub fn get_reason(&self) -> &str {
    match self {
      Fault::MissingArguments => "Not enough input parameters passed. Please specify first the file to read, then the task to be run (e.g. './input 1')",
      Fault::CannotReadInputFile => "Unable to read provided file. Make sure the file exists and permission to read is given."
    }
  } 
}