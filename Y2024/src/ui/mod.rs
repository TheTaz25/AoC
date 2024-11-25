pub mod app_ui;
pub mod days;
pub mod data_entry;

pub enum SubUiResponse {
  Exit,
  SelectDay(usize),
  AcceptInput(String),
  None,
}