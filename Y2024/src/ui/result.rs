use ratatui::{
  buffer::Buffer, crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Stylize, widgets::{Block, Paragraph, Widget}
};

use crate::tasks::TaskData;

use super::SubUiResponse;

// ToDo: Debug-View in Bottom Part

#[derive(Clone)]
pub struct UiResult {
  data: Option<TaskData>,
}

impl Default for UiResult {
  fn default() -> Self {
    Self {
      data: None,
    }
  }
}

impl Widget for &mut UiResult {
  fn render(self, area: Rect, buf: &mut Buffer)
      where
          Self: Sized {
    let [top, center, _bottom] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(1),
    ]).areas(area);
    let [top_left, top_right] = Layout::horizontal([
      Constraint::Fill(1),
      Constraint::Fill(1),
    ]).areas(top);

    match &self.data {
      Some(data) => {
        let (task1, task2) = data.get_results();
        let checkpoints = data.get_timestamps();
        

        // Render Result - Part 1 of Day
        Widget::render(
          Paragraph::new(task1)
          .alignment(Alignment::Center)
          .block(Block::bordered().green().title("  Calculation Part 1  ")),
          top_left, buf);

        // Render Result - Part 2 of Day
        Widget::render(
          Paragraph::new(task2)
            .alignment(Alignment::Center)
            .block(Block::bordered().green().title("  Calculation Part 2  ")),
            top_right, buf);

        // Render Some Stats (Duration between custom defined steps)
        let timeslot_chunks = Layout::default()
          .direction(Direction::Horizontal)
          .constraints(
            (0..checkpoints.len() - 1)
              .into_iter()
              .map(|_| Constraint::Fill(1))
              .collect::<Vec<_>>()
          ).split(center);

        for slot in 1..checkpoints.len() {
          let (_, previous) = checkpoints.get(slot - 1).unwrap();
          let (title, current) = checkpoints.get(slot).unwrap();
          Widget::render(
            Paragraph::new(format!("{}ms", current.duration_since(*previous).as_millis()))
            .block(Block::bordered().yellow().title(format!("  {}  ", title))), *timeslot_chunks.get(slot - 1).unwrap(), buf
          );
        }
      },
      None => {
        let paragraph = Paragraph::new("Failed to Load Calculation Results")
          .block(Block::bordered().red().title("  Calculation Result  "));
        Widget::render(paragraph, area, buf);
      }
    }
  }
}

impl UiResult {
  pub fn send_event(&mut self, key: KeyEvent) -> SubUiResponse {
    if key.kind != KeyEventKind::Press {
      return SubUiResponse::None;
    }

    let response = match key.code {
      KeyCode::Char('q') | KeyCode::Esc => SubUiResponse::Exit,
      _ => SubUiResponse::None
    };

    response
  }

  pub fn set_result(&mut self, res: Option<TaskData>) {
    self.data = res;
  }
}