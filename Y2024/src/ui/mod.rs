pub mod app_ui;
pub mod days;
pub mod data_entry;
pub mod runner;
pub mod result;

pub enum SubUiResponse {
  Exit,
  SelectDay(usize),

  AcceptInput(String),

  RunTask,
  TaskDone,
  None,
}