use ratatui::{
  buffer::Buffer, crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Stylize, widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget}
};

use crate::tasks::TaskData;

use super::SubUiResponse;

// ToDo: Debug-View in Bottom Part

#[derive(Clone)]
pub struct UiResult {
  data: Option<TaskData>,
  scroll_state: usize,
  scroll_bar_state: ScrollbarState,
  area_height: usize,
  max_scroll: usize,
}

impl Default for UiResult {
  fn default() -> Self {
    Self {
      data: None,
      scroll_bar_state: ScrollbarState::new(1),
      scroll_state: 0,
      area_height: 1,
      max_scroll: 0,
    }
  }
}

const PAGE_OFFSET: usize = 2;

impl Widget for &mut UiResult {
  fn render(self, area: Rect, buf: &mut Buffer)
      where
          Self: Sized {
    
    let [top, center, bottom] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(1),
    ]).areas(area);
    let [top_left, top_right] = Layout::horizontal([
      Constraint::Fill(1),
      Constraint::Fill(1),
    ]).areas(top);

    self.area_height = bottom.height as usize;

    match &self.data {
      Some(data) => {
        let (task1, task2) = data.get_results();
        let checkpoints = data.get_timestamps();
        let logs = data.get_logs();        

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

        Widget::render(
          Paragraph::new(logs.join("\n"))
            .block(Block::bordered().red().title("  Calculation Logs "))
            .scroll((self.scroll_state as u16, 0)),
            bottom, buf
        );
        StatefulWidget::render(Scrollbar::new(ScrollbarOrientation::VerticalRight)
          .begin_symbol(Some("^"))
          .end_symbol(Some("v")),
          bottom, buf, &mut self.scroll_bar_state,
        );
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

    let mut scroll_adjust: i16 = 0;

    let response = match key.code {
      KeyCode::Char('q') | KeyCode::Esc => SubUiResponse::Exit,
      KeyCode::Char('j') | KeyCode::Down => {
        scroll_adjust = 1;
        SubUiResponse::None
      },
      KeyCode::Char('k') | KeyCode::Up => {
        scroll_adjust = -1;
        SubUiResponse::None
      },
      KeyCode::PageDown => {
        scroll_adjust = self.area_height as i16;
        SubUiResponse::None
      },
      KeyCode::PageUp => {
        scroll_adjust = -(self.area_height as i16);
        SubUiResponse::None
      }
      _ => SubUiResponse::None
    };

    if scroll_adjust != 0 {
      self.scroll_state = self.scroll_state
        .saturating_add_signed(scroll_adjust.into())
        .clamp(0, self.max_scroll - self.area_height + PAGE_OFFSET);

      self.scroll_bar_state = self.scroll_bar_state.position(self.scroll_state);
    }

    response
  }

  pub fn set_result(&mut self, res: Option<TaskData>) {
    self.data = res.clone();
    if res.is_some() {
      let max_scroll = res.unwrap().get_logs().len();

      self.scroll_bar_state = self.scroll_bar_state.content_length(max_scroll.saturating_sub(self.area_height + PAGE_OFFSET));
      self.scroll_state = 0;
      self.scroll_bar_state.first();

      self.max_scroll = max_scroll;
    }
  }
}