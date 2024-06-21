use std::{
  io::Stdout,
  sync::{Arc, Mutex},
};

use napi::Result;
use ratatui::{
  backend::CrosstermBackend,
  layout::Rect,
  widgets::{Paragraph, Widget},
  Terminal,
};

#[napi(js_name = "Frame")]
pub struct JSFrame {
  terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>,
}

impl JSFrame {
  pub fn new(terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>) -> JSFrame {
    JSFrame { terminal }
  }
}

#[napi]
impl JSFrame {
  #[napi(constructor)]
  pub fn __() -> Result<Self> {
    Err(napi::Error::from_reason(
      "This constructor should not be called directly.",
    ))
  }

  #[napi]
  pub fn render_widget_(&mut self, paragraph: &JSParagraph, area: &JSRect) {
    let paragraph = Paragraph::new(paragraph.text.clone());

    let mut terminal = self.terminal.lock().unwrap();
    let buffer = terminal.current_buffer_mut();
    paragraph.render(area.into(), buffer);
  }

  #[napi]
  pub fn size(&self) -> JSRect {
    let mut terminal = self.terminal.lock().unwrap();
    terminal.get_frame().size().into()
  }
}

#[napi(constructor, js_name = "Paragraph")]
pub struct JSParagraph {
  pub text: String,
}

#[napi]
pub struct JSRect {
  x: u16,
  y: u16,
  width: u16,
  height: u16,
}

impl JSRect {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> JSRect {
    JSRect {
      x,
      y,
      width,
      height,
    }
  }
}

impl From<&JSRect> for Rect {
  fn from(rect: &JSRect) -> Rect {
    Rect::new(rect.x, rect.y, rect.width, rect.height)
  }
}

impl From<Rect> for JSRect {
  fn from(rect: Rect) -> JSRect {
    JSRect::new(rect.x, rect.y, rect.width, rect.height)
  }
}
