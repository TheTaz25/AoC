pub trait TaskRunner {
  fn new () -> Self where Self: Sized;
  fn run(self) -> Result<String, ()>;
  fn input(&mut self, data: String);
}

pub mod t0;