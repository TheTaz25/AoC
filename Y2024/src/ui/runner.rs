
use ratatui::widgets::{Paragraph, Widget};
use ratatui::style::Stylize;

use crate::tasks::{t1, TaskData};

#[derive(Clone)]
pub struct RunnerUi {
  task_data: Option<String>,
  selected_day: Option<usize>,
  pub result_data: Option<TaskData>,
}

impl Default for RunnerUi {
  fn default() -> Self {
      Self {
        task_data: None,
        selected_day: None,
        result_data: None,
      }
  }
}

impl RunnerUi {
  pub fn set_data(&mut self, data: String) {
    self.task_data = Some(data);
  }

  pub fn set_selected_day(&mut self, day: usize) {
    self.selected_day = Some(day);
  }

  pub fn execute(&mut self) {
    let data = self.task_data.clone();
    match self.selected_day {
      Some(0) => {
        let results = t1::execute_day(
          data.unwrap(),
        );

        self.result_data = Some(results);
      },
      _ => {},
    };
  }

  pub fn is_ready(&self) -> bool {
    self.selected_day.is_some() && self.task_data.is_some()
  }
}

impl Widget for &mut RunnerUi {
  fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
      where
          Self: Sized {

    Paragraph::new(format!("Running..."))
      .bold()
      .centered()
      .render(area, buf);
  }
}