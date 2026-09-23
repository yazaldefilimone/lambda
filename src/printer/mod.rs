use std::fmt::Write;

use crate::{
  context::Context,
  core::{ApplyId, LambdaId, TermId, TypeId},
  symbol::SymbolId,
};

const CANDIDATES: &[&str] = &[
  "x", "y", "z", "w", "v", "u", "t", "s", "r", "q", "p", "a", "b", "c", "d", "e", "f", "g", "h",
  "i", "j", "k", "l", "m", "n", "o",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
  pub collapse: bool,
}

impl Default for Options {
  fn default() -> Self {
    Self { collapse: false }
  }
}

pub struct Printer<'a> {
  ctx: &'a Context,
  options: Options,
  free_vars_mask: u32,
}

impl<'a> Printer<'a> {
  pub fn new(ctx: &'a Context) -> Self {
    let mut free_vars_mask = 0u32;
    for (i, &cand) in CANDIDATES.iter().enumerate() {
      let mut iterator = ctx.terms.free_variables.iter();
      if iterator.any(|&sym| ctx.symbols.resolve(sym) == cand) {
        free_vars_mask |= 1 << i;
      }
    }
    Self { ctx, options: Options::default(), free_vars_mask }
  }

  #[allow(dead_code)]
  pub fn with_options(ctx: &'a Context, options: Options) -> Self {
    let mut printer = Self::new(ctx);
    printer.options = options;
    printer
  }

  pub fn print_term(&self, term_id: TermId) -> String {
    let mut out = String::with_capacity(64);
    let mut names = Vec::with_capacity(16);
    self.format_term(term_id, &mut names, &mut out);
    out
  }

  pub fn print_term_in_scope(&self, term_id: TermId, scope: &[LambdaId]) -> String {
    let mut out = String::with_capacity(64);
    let mut names = Vec::with_capacity(scope.len() + 16);
    for &lam_id in scope {
      let name = self.pick_name(&names, Some(lam_id));
      names.push(name);
    }
    self.format_term(term_id, &mut names, &mut out);
    out
  }

  #[allow(dead_code)]
  pub fn print_type(&self, type_id: TypeId) -> String {
    let mut out = String::new();
    self.format_type(type_id, &mut out);
    out
  }

  fn resolve_symbol(&self, symbol: SymbolId) -> &'a str {
    self.ctx.symbols.resolve(symbol)
  }

  fn pick_name(&self, names: &[&str], lam_id: Option<LambdaId>) -> &'a str {
    if let Some(id) = lam_id
      && let Some(sym) = self.ctx.terms.parameter(id)
    {
      let orig = self.resolve_symbol(sym);
      if !names.contains(&orig) {
        return orig;
      }
    }

    for (i, &candidate) in CANDIDATES.iter().enumerate() {
      if (self.free_vars_mask & (1 << i)) == 0 && !names.contains(&candidate) {
        return candidate;
      }
    }
    CANDIDATES[names.len() % CANDIDATES.len()]
  }

  fn format_term(&self, term_id: TermId, names: &mut Vec<&'a str>, out: &mut String) {
    match term_id.kind() {
      crate::core::TermKind::Variable(var_id) => {
        let var = &self.ctx.terms.variables[var_id];
        let index = var.index as usize;
        if index < names.len() {
          let name = names[names.len() - 1 - index];
          out.push_str(name);
        } else {
          let free_idx = index - names.len();
          if let Some(&sym) = self.ctx.terms.free_variables.get(free_idx) {
            out.push_str(self.resolve_symbol(sym));
          } else {
            let _ = write!(out, "_{index}");
          }
        }
      },
      crate::core::TermKind::Lambda(lam_id) => {
        if self.options.collapse {
          self.format_collapsed_lambda(lam_id, names, out);
        } else {
          let lam = &self.ctx.terms.lambdas[lam_id];
          let annotation = self.ctx.terms.annotation(lam_id);
          let name = self.pick_name(names, Some(lam_id));
          out.push('λ');
          names.push(name);
          self.format_parameter(name, annotation, out);
          out.push_str(". ");
          self.format_term(lam.body, names, out);
          names.pop();
        }
      },
      crate::core::TermKind::Apply(apply_id) => {
        self.format_apply(apply_id, names, out);
      },
    }
  }

  fn format_collapsed_lambda(
    &self,
    mut current_id: LambdaId,
    names: &mut Vec<&'a str>,
    out: &mut String,
  ) {
    out.push('λ');
    let mut first = true;
    let mut pushed_count = 0;

    loop {
      let lam = &self.ctx.terms.lambdas[current_id];
      let annotation = self.ctx.terms.annotation(current_id);
      let name = self.pick_name(names, Some(current_id));
      names.push(name);
      pushed_count += 1;

      if !first {
        out.push(' ');
      }
      self.format_parameter(name, annotation, out);
      first = false;

      if let Some(next_id) = lam.body.as_lambda() {
        current_id = next_id;
      } else {
        out.push_str(". ");
        self.format_term(lam.body, names, out);
        break;
      }
    }

    for _ in 0..pushed_count {
      names.pop();
    }
  }

  fn format_parameter(&self, name: &str, annotation: Option<TypeId>, out: &mut String) {
    out.push_str(name);
    if let Some(type_id) = annotation {
      out.push_str(": ");
      self.format_type(type_id, out);
    }
  }

  fn format_apply(&self, apply_id: ApplyId, names: &mut Vec<&'a str>, out: &mut String) {
    let apply = &self.ctx.terms.applies[apply_id];

    if apply.function.is_lambda() {
      out.push('(');
      self.format_term(apply.function, names, out);
      out.push(')');
    } else {
      self.format_term(apply.function, names, out);
    }

    out.push(' ');

    if apply.argument.is_variable() {
      self.format_term(apply.argument, names, out);
    } else {
      out.push('(');
      self.format_term(apply.argument, names, out);
      out.push(')');
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
