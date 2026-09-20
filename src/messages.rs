#![allow(dead_code)]

use crate::source::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
  Info,
  Warning,
  Error,
}

impl Severity {
  pub fn name(self) -> &'static str {
    match self {
      Severity::Info => "info",
      Severity::Warning => "warning",
      Severity::Error => "error",
    }
  }
}

#[derive(Debug)]
pub struct Message {
  pub severity: Severity,
  pub text: String,
  pub span: Option<Span>,
  pub notes: Vec<Note>,
}

impl Message {
  pub fn info(text: impl Into<String>) -> Self {
    Self { severity: Severity::Info, text: text.into(), span: None, notes: vec![] }
  }

  pub fn warning(text: impl Into<String>) -> Self {
    Self { severity: Severity::Warning, text: text.into(), span: None, notes: vec![] }
  }

  pub fn error(text: impl Into<String>) -> Self {
    Self { severity: Severity::Error, text: text.into(), span: None, notes: vec![] }
  }

  pub fn with_span(mut self, span: Span) -> Self {
    self.span = Some(span);
    self
  }

  pub fn with_note(mut self, note: Note) -> Self {
    self.notes.push(note);
    self
  }
}

#[derive(Debug)]
pub struct Note {
  pub text: String,
  pub span: Option<Span>,
}

impl Note {
  pub fn new(text: impl Into<String>, span: Option<Span>) -> Self {
    Self { text: text.into(), span }
  }
}

#[derive(Debug)]
pub struct Messages {
  pub list: Vec<Message>,
}

impl Messages {
  pub fn new() -> Self {
    Self { list: Vec::new() }
  }

  pub fn add(&mut self, message: Message) {
    self.list.push(message);
  }

  pub fn error(&mut self, text: impl Into<String>, span: Option<Span>) {
    self.list.push(Message::error(text).with_span_if_some(span));
  }

  pub fn warning(&mut self, text: impl Into<String>, span: Option<Span>) {
    self
      .list
      .push(Message::warning(text).with_span_if_some(span));
  }

  pub fn info(&mut self, text: impl Into<String>, span: Option<Span>) {
    self.list.push(Message::info(text).with_span_if_some(span));
  }

  pub fn has_errors(&self) -> bool {
    self
      .list
      .iter()
      .any(|message| message.severity == Severity::Error)
  }

  pub fn count(&self) -> usize {
    self.list.len()
  }
}

impl Message {
  fn with_span_if_some(mut self, span: Option<Span>) -> Self {
    self.span = span;
    self
  }
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    $crate::messages::Message::error(
      format!($($arg)*)
    )
  }
}

#[macro_export]
macro_rules! warning {
  ($($arg:tt)*) => {
    $crate::messages::Message::warning(
      format!($($arg)*)
    )
  }
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
    $crate::messages::Message::info(
      format!($($arg)*)
    )
  }
}

#[macro_export]
macro_rules! note {
  ($span:expr, $($arg:tt)*) => {
    $crate::messages::Note::new(
      format!($($arg)*),
      Some($span),
    )
  }
}
