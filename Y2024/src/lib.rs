pub mod ui;
pub mod tasks;
pub mod utils;

pub enum RunnerState {
  Ready,
  Working(String),
  Done((String, String))
}