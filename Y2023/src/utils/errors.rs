#[derive(Debug)]
pub enum Fault {
  MissingArguments,
  CannotReadInputFile,
  UnknownError,
  FailedToExtractDigits,
  ConversionError,
  D2GameIdNotFound,
}

impl Fault {
  pub fn get_reason(&self) -> &str {
    match self {
      Fault::MissingArguments => "Not enough input parameters passed. Please specify first the file to read, then the task to be run (e.g. './input 1')",
      Fault::CannotReadInputFile => "Unable to read provided file. Make sure the file exists and permission to read is given.",
      Fault::FailedToExtractDigits => "Unable to extract necessary digits from string",
      Fault::UnknownError => "Could not complete task",
      Fault::D2GameIdNotFound => "[DAY 2] Unable to find game ID",
      Fault::ConversionError => "Failed to convert unit"
    }
  } 
}