use crate::{
  context::Context,
  core::{ApplyId, LambdaId, TermId, TypeId},
  symbol::SymbolId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
  pub collapse: bool,
}

pub type PrintOptions = Options;

impl Default for Options {
  fn default() -> Self {
    Self { collapse: false }
  }
}

pub struct Printer<'a> {
  ctx: &'a Context,
  options: Options,
}

impl<'a> Printer<'a> {
  pub fn new(ctx: &'a Context) -> Self {
    Self { ctx, options: Options::default() }
  }

  pub fn with_options(ctx: &'a Context, options: Options) -> Self {
    Self { ctx, options }
  }

  pub fn print_term(&self, term_id: TermId) -> String {
    let mut out = String::new();
    self.format_term(term_id, &mut out);
    out
  }

  pub fn print_type(&self, type_id: TypeId) -> String {
    let mut out = String::new();
    self.format_type(type_id, &mut out);
    out
  }

  fn resolve_symbol(&self, symbol: SymbolId) -> &str {
    self.ctx.symbols.resolve(symbol)
  }

  fn format_term(&self, term_id: TermId, out: &mut String) {
    match term_id {
      TermId::Variable(var_id) => {
        let var = &self.ctx.terms.variables[var_id];
        out.push_str(self.resolve_symbol(var.name));
      },
      TermId::Lambda(lam_id) => {
        if self.options.collapse {
          self.format_collapsed_lambda(lam_id, out);
        } else {
          let lam = &self.ctx.terms.lambdas[lam_id];
          out.push('λ');
          self.format_parameter(lam.parameter, lam.annotation, out);
          out.push_str(". ");
          self.format_term(lam.body, out);
        }
      },
      TermId::Apply(apply_id) => {
        self.format_apply(apply_id, out);
      },
    }
  }

  fn format_collapsed_lambda(&self, mut current_id: LambdaId, out: &mut String) {
    out.push('λ');
    let mut first = true;

    loop {
      let lam = &self.ctx.terms.lambdas[current_id];
      if !first {
        out.push(' ');
      }
      self.format_parameter(lam.parameter, lam.annotation, out);
      first = false;

      match lam.body {
        TermId::Lambda(next_id) => {
          current_id = next_id;
        },
        other => {
          out.push_str(". ");
          self.format_term(other, out);
          break;
        },
      }
    }
  }

  fn format_parameter(&self, name: SymbolId, annotation: Option<TypeId>, out: &mut String) {
    out.push_str(self.resolve_symbol(name));
    if let Some(type_id) = annotation {
      out.push_str(": ");
      self.format_type(type_id, out);
    }
  }

  fn format_apply(&self, apply_id: ApplyId, out: &mut String) {
    let apply = &self.ctx.terms.applies[apply_id];

    match apply.function {
      TermId::Lambda(_) => {
        out.push('(');
        self.format_term(apply.function, out);
        out.push(')');
      },
      _ => {
        self.format_term(apply.function, out);
      },
    }

    out.push(' ');

    match apply.argument {
      TermId::Variable(_) => {
        self.format_term(apply.argument, out);
      },
      _ => {
        out.push('(');
        self.format_term(apply.argument, out);
        out.push(')');
      },
    }
  }

  fn format_type(&self, type_id: TypeId, out: &mut String) {
    match type_id {
      TypeId::Function(func_id) => {
        let func = &self.ctx.types.functions[func_id];
        match func.parameter {
          TypeId::Function(_) => {
            out.push('(');
            self.format_type(func.parameter, out);
            out.push(')');
          },
          _ => {
            self.format_type(func.parameter, out);
          },
        }

        out.push_str(" -> ");
        self.format_type(func.result, out);
      },
      TypeId::Apply(apply_id) => {
        let apply = &self.ctx.types.applies[apply_id];

        match apply.constructor {
          TypeId::Function(_) => {
            out.push('(');
            self.format_type(apply.constructor, out);
            out.push(')');
          },
          _ => {
            self.format_type(apply.constructor, out);
          },
        }

        out.push(' ');

        match apply.argument {
          TypeId::Named(_) | TypeId::Variable(_) => {
            self.format_type(apply.argument, out);
          },
          _ => {
            out.push('(');
            self.format_type(apply.argument, out);
            out.push(')');
          },
        }
      },
      TypeId::Named(named_id) => {
        let named = &self.ctx.types.named[named_id];
        out.push_str(self.resolve_symbol(named.name));
      },
      TypeId::Variable(var_id) => {
        let var = &self.ctx.types.variables[var_id];
        out.push_str(self.resolve_symbol(var.name));
      },
    }
  }
}
