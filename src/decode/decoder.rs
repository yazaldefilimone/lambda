use super::church;
use crate::{
  context::Context,
  core::TermId,
  decode::ValueId,
  error,
  result::{self, Failed},
};

pub struct Decoder<'a> {
  ctx: &'a mut Context,
  values: super::value::Values,
}

impl<'a> Decoder<'a> {
  pub fn new(ctx: &'a mut Context) -> Self {
    let values = super::value::Values::new();
    Self { ctx, values }
  }

  pub fn church(&mut self, term: TermId) -> result::Result<ValueId> {
    let decoded = church::decode(self.ctx, &mut self.values, term)?;
    if let Some(value) = decoded {
      return Ok(value);
    }
    let message = error!("cannot decode value");
    self.ctx.messages.add(message);
    let err = Failed::Abort;
    Err(err)
  }

  pub fn print(&self, id: ValueId) -> String {
    let printed = self.values.print(id);
    printed
  }

  #[allow(dead_code)]
  pub fn scott(&mut self, _term: TermId) -> result::Result<()> {
    todo!("'scott' encoding not yet supported")
  }

  #[allow(dead_code)]
  pub fn boehm(&mut self, _term: TermId) -> result::Result<()> {
    todo!("'boehm' encoding not yet supported")
  }
}
