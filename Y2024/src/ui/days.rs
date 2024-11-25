use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget};
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;
use ratatui::style::{Style, Modifier, Color, palette::tailwind::{BLUE, AMBER}, Stylize};

use super::SubUiResponse;

#[derive(Clone)]
pub struct DayUI <'a>{
  day_state: ListState,
  days: Vec<ListItem<'a>>
}

fn get_days() -> Vec<ListItem<'static>> {
  (1..25).map(|day| ListItem::from(format!("Day {}", day.to_string()))).collect()
}

impl Default for DayUI<'_> {
  fn default() -> Self {
    let mut day_state = ListState::default();
    day_state.select_first();
      Self {
        day_state,
        days: get_days(),
      }
  }
}

const DAY_HEADER_STYLE: Style = Style::new().fg(BLUE.c800).bg(AMBER.c200);
const DAY_BG_STYLE: Color = AMBER.c950;
const DAY_SELECTED_STYLE: Style = Style::new().bg(AMBER.c400).add_modifier(Modifier::BOLD);

impl Widget for &mut DayUI<'_> {
  fn render(self, area: Rect, buf: &mut Buffer)
      where
          Self: Sized {
      let block = Block::new()
        .title(Line::raw("  AoC - Day Selection  "))
        .borders(Borders::ALL)
        .border_set(symbols::border::PLAIN)
        .border_style(DAY_HEADER_STYLE)
        .bg(DAY_BG_STYLE);

      let list = List::new(self.days.clone())
        .block(block)
        .scroll_padding(4)
        .highlight_style(DAY_SELECTED_STYLE)
        .highlight_symbol(">  ")
        .highlight_spacing(HighlightSpacing::Always);

      StatefulWidget::render(list, area, buf, &mut self.day_state);
  }
}

impl DayUI<'_> {
  pub fn send_event(&mut self, key: KeyEvent) -> SubUiResponse {
    if key.kind != KeyEventKind::Press {
      return SubUiResponse::None;
    }

    let response = match key.code {
      KeyCode::Char('q') | KeyCode::Esc => SubUiResponse::Exit,
      KeyCode::Char('j') | KeyCode::Down => {
        self.day_state.select_next();
        SubUiResponse::None
      },
      KeyCode::Char('k') | KeyCode::Up => {
        self.day_state.select_previous();
        SubUiResponse::None
      },
      KeyCode::Enter => SubUiResponse::SelectDay(self.day_state.selected().expect("Expected Day to be selected")),
      _ => SubUiResponse::None
    };

    response
  }
}