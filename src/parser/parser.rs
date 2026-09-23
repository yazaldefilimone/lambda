use crate::{
  core::*,
  error,
  messages::Messages,
  parser::{
    reader::Reader,
    tokens::{Token, kind::TokenKind},
  },
  result,
  symbol::SymbolId,
};

pub struct Parser<'a> {
  pub reader: Reader<'a>,
  pub messages: &'a mut Messages,
  pub terms: &'a mut Term,
  pub types: &'a mut Type,
  pub scope: Vec<SymbolId>,
}

impl<'a> Parser<'a> {
  pub fn new(
    tokens: &'a [Token],
    messages: &'a mut Messages,
    terms: &'a mut Term,
    types: &'a mut Type,
  ) -> Self {
    Self { reader: Reader::new(tokens), messages, terms, types, scope: Vec::new() }
  }

  pub fn parse_module(&mut self) -> result::Result<TermId> {
    let term = self.parse_term()?;
    self.reader.expect(TokenKind::Eof, self.messages)?;
    Ok(term)
  }

  pub fn parse_term(&mut self) -> result::Result<TermId> {
    if self.reader.check(TokenKind::Lambda) {
      self.parse_lambda()
    } else {
      self.parse_application()
    }
  }

  pub fn parse_lambda(&mut self) -> result::Result<TermId> {
    self.reader.expect(TokenKind::Lambda, self.messages)?;

    let mut parameters = Vec::new();
    parameters.push(self.parse_parameter()?);

    while matches!(self.reader.current(), TokenKind::Identifier(_)) {
      parameters.push(self.parse_parameter()?);
    }

    self.reader.expect(TokenKind::Dot, self.messages)?;

    for &(param_name, _) in &parameters {
      self.scope.push(param_name);
    }

    let body = self.parse_term()?;

    for _ in 0..parameters.len() {
      self.scope.pop();
    }

    let mut current = body;
    for (param_name, annotation) in parameters.into_iter().rev() {
      let lambda = Lambda { body: current };
      current = self
        .terms
        .add_annotated(lambda, Some(param_name), annotation);
    }

    Ok(current)
  }

  pub fn parse_application(&mut self) -> result::Result<TermId> {
    let mut left = self.parse_atom()?;

    while self.is_atom_start() {
      let right = self.parse_atom()?;
      let apply = Apply { function: left, argument: right };
      left = self.terms.add_apply(apply);
    }

    Ok(left)
  }

  pub fn parse_atom(&mut self) -> result::Result<TermId> {
    match self.reader.current() {
      TokenKind::Identifier(symbol) => {
        self.reader.advance();
        let index = if let Some(pos) = self.scope.iter().rev().position(|&s| s == symbol) {
          pos as u32
        } else if let Some(pos) = self.terms.free_variables.iter().position(|&s| s == symbol) {
          (self.scope.len() + pos) as u32
        } else {
          let pos = self.terms.free_variables.len();
          self.terms.free_variables.push(symbol);
          (self.scope.len() + pos) as u32
        };
        let variable = Variable { index };
        Ok(self.terms.add_variable(variable))
      },
      TokenKind::LeftParen => {
        self.reader.advance();
        let term = self.parse_term()?;
        self.reader.expect(TokenKind::RightParen, self.messages)?;
        Ok(term)
      },
      kind => {
        let token = self.reader.peek().cloned();
        let message = error!("expected atom, found '{}'", kind);
        if let Some(tok) = token {
          self.messages.add(message.with_span(tok.span));
        } else {
          self.messages.add(message);
        }
        Err(result::Failed::Recover)
      },
    }
  }

  pub fn parse_parameter(&mut self) -> result::Result<(SymbolId, Option<TypeId>)> {
    let name = self.reader.expect_identifier(self.messages)?;
    let annotation =
      if self.reader.consume(TokenKind::Colon) { Some(self.parse_type()?) } else { None };
    Ok((name, annotation))
  }

  pub fn parse_type(&mut self) -> result::Result<TypeId> {
    self.parse_function_type()
  }

  pub fn parse_function_type(&mut self) -> result::Result<TypeId> {
    let param = self.parse_apply_type()?;

    if self.reader.consume(TokenKind::Arrow) {
      let res = self.parse_function_type()?;
      let function = FunctionType { parameter: param, result: res };
      Ok(self.types.add_function(function))
    } else {
      Ok(param)
    }
  }

  pub fn parse_apply_type(&mut self) -> result::Result<TypeId> {
    let mut left = self.parse_primary_type()?;

    while self.is_primary_type_start() {
      let right = self.parse_primary_type()?;
      let apply = ApplyType { constructor: left, argument: right };
      left = self.types.add_apply(apply);
    }

    Ok(left)
  }

  pub fn parse_primary_type(&mut self) -> result::Result<TypeId> {
    match self.reader.current() {
      TokenKind::Identifier(symbol) => {
        self.reader.advance();
        let named = NamedType { name: symbol };
        Ok(self.types.add_named(named))
      },
      TokenKind::LeftParen => {
        self.reader.advance();
        let ty = self.parse_type()?;
        self.reader.expect(TokenKind::RightParen, self.messages)?;
        Ok(ty)
      },
      kind => {
        let token = self.reader.peek().cloned();
        let message = error!("expected type, found '{}'", kind);
        if let Some(tok) = token {
          self.messages.add(message.with_span(tok.span));
        } else {
          self.messages.add(message);
        }
        Err(result::Failed::Recover)
      },
    }
  }

  fn is_atom_start(&self) -> bool {
    matches!(self.reader.current(), TokenKind::Identifier(_) | TokenKind::LeftParen)
  }

  fn is_primary_type_start(&self) -> bool {
    match self.reader.current() {
      TokenKind::LeftParen => true,
      TokenKind::Identifier(_) => {
        if let Some(next_token) = self.reader.peek_offset(1) {
          if matches!(next_token.kind, TokenKind::Colon) {
            return false;
          }
        }
        true
      },
      _ => false,
    }
  }
}
