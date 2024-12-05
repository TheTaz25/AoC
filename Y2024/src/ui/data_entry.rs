use ratatui::{
  buffer::Buffer,
  layout::Rect,
  crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
  style::Stylize,
  widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget}
};

use super::SubUiResponse;
use arboard::{Clipboard, Error};

#[derive(Clone)]
pub struct DataEntry {
  content: String,
  scroll_state: usize,
  scroll_bar_state: ScrollbarState,
  area_height: usize,
  max_scroll: usize,
}

impl Default for DataEntry {
  fn default() -> Self {
      Self {
        content: String::from("Hallo,\ndas ist ein Test"),

        scroll_bar_state: ScrollbarState::new(1),

        scroll_state: 0,
        area_height: 1,
        max_scroll: 0,
      }
  }
}

const PAGE_OFFSET: usize = 2;

impl Widget for &mut DataEntry {
  fn render(self, area: Rect, buf: &mut Buffer)
      where
        Self: Sized {

    self.area_height = area.height as usize;

    let paragraph = Paragraph::new(self.content.clone())
      .block(Block::bordered().gray().title("AoC Input"))
      .scroll((self.scroll_state as u16, 0));

    Widget::render(paragraph, area, buf);
    StatefulWidget::render(Scrollbar::new(ScrollbarOrientation::VerticalRight)
      .begin_symbol(Some("^"))
      .end_symbol(Some("v")),

          area, buf, &mut self.scroll_bar_state,
    )
  }
}

impl DataEntry {
  fn paste_content(&mut self) -> Result<(),
 Error> {
    let mut board = Clipboard::new()?;
    let content = String::from(board.get_text()?);
    self.content = content.trim().to_string();
    
    let max_scroll = self.content.lines().collect::<Vec<&str>>().len();
    
    self.scroll_bar_state = self.scroll_bar_state.content_length(max_scroll.saturating_sub(self.area_height + PAGE_OFFSET));
    self.scroll_state = 0;
    self.scroll_bar_state.first();

    self.max_scroll = max_scroll;
    Ok(())
  }

  pub fn send_event(&mut self, key: KeyEvent) -> SubUiResponse {
    if key.kind != KeyEventKind::Press {
      return SubUiResponse::None;
    }

    let mut scroll_adjust: i16 = 0;

    let response = match key.code {
      KeyCode::Char('p') => {
        let _ = self.paste_content();
        // if r.is_err() {
        //   println!("{:?}", r.err());
        // }
        SubUiResponse::None
      },
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
      },
      KeyCode::Enter => {
        let to_submit = self.content.to_string();
        self.content = String::new();
        self.scroll_bar_state = ScrollbarState::new(1);
        self.max_scroll = 0;
        self.scroll_state = 0;
        self.area_height = 1;
        SubUiResponse::AcceptInput(to_submit)
      }
      KeyCode::Char('q') | KeyCode::Esc => SubUiResponse::Exit,
      _ => SubUiResponse::None,
    };

    if scroll_adjust != 0 {
      self.scroll_state = self.scroll_state
        .saturating_add_signed(scroll_adjust.into())
        .clamp(0, self.max_scroll - self.area_height + PAGE_OFFSET);
  
      self.scroll_bar_state = self.scroll_bar_state.position(self.scroll_state);
    }

    response
  }
}