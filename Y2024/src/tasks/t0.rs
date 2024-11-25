use super::TaskRunner;

pub struct T0 {
  input: String
}

impl TaskRunner for T0 {
  fn new () -> Self {
    Self { input: "".to_string() }
  }

  fn input(&mut self, data: String) {
    self.input = data;
  }

  fn run(self) -> Result<String, ()> {
    Ok("".to_string())
  }
}
