use std::{
  io::Stdout,
  sync::{Arc, Mutex},
};

use crossterm::{
  terminal::{disable_raw_mode, enable_raw_mode},
  ExecutableCommand,
};
use napi::threadsafe_function::{ErrorStrategy::Fatal, ThreadsafeFunction};
use ratatui::{
  backend::{Backend, CrosstermBackend},
  Terminal,
};

use crate::frame::JSFrame;

#[napi(js_name = "Stdout")]
pub enum JSStdout {
  EnterAlternateScreen,
  LeaveAlternateScreen,
}

#[napi(js_name = "Terminal")]
pub struct InnerTerminal {
  terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>,
  stdout: Arc<Mutex<Stdout>>,
}

#[napi]
impl InnerTerminal {
  #[napi(constructor)]
  pub fn new() -> Self {
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend).expect("Failed to create terminal");

    enable_raw_mode().expect("Failed to enable raw mode");

    Self {
      terminal: Arc::new(Mutex::new(terminal)),
      stdout: Arc::new(Mutex::new(std::io::stdout())),
    }
  }

  /// # Safety
  /// Trust me bro!
  #[napi]
  pub async unsafe fn draw(
    &mut self,
    #[napi(ts_arg_type = "(frame: Frame) => void")] callback: ThreadsafeFunction<JSFrame, Fatal>,
  ) {
    callback
      .call_async::<()>(JSFrame::new(Arc::clone(&self.terminal)))
      .await
      .expect("Failed to call callback");

    let mut terminal = self.terminal.lock().unwrap();
    terminal.flush().expect("Failed to flush terminal");
    terminal.hide_cursor().expect("Failed to hide cursor");
    terminal.swap_buffers();
    terminal
      .backend_mut()
      .flush()
      .expect("Failed to flush backend");
  }

  #[napi]
  pub fn setup(&mut self, state: JSStdout) {
    let mut stdout = self.stdout.lock().unwrap();
    enable_raw_mode().expect("Failed to enable raw mode");

    match state {
      JSStdout::EnterAlternateScreen => stdout.execute(crossterm::terminal::EnterAlternateScreen),
      JSStdout::LeaveAlternateScreen => stdout.execute(crossterm::terminal::LeaveAlternateScreen),
    }
    .expect("Failed to setup terminal");
  }

  #[napi]
  pub fn restore(&mut self, state: JSStdout) {
    let mut stdout = self.stdout.lock().unwrap();

    disable_raw_mode().expect("Failed to disable raw mode");
    match state {
      JSStdout::EnterAlternateScreen => stdout.execute(crossterm::terminal::EnterAlternateScreen),
      JSStdout::LeaveAlternateScreen => stdout.execute(crossterm::terminal::LeaveAlternateScreen),
    }
    .expect("Failed to setup terminal");
  }
}

impl Default for InnerTerminal {
  fn default() -> Self {
    Self::new()
  }
}
