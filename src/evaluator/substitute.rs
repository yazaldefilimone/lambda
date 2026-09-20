use crate::{
  context::Context,
  core::{Apply, ApplyId, Lambda, LambdaId, TermId, VariableId},
  evaluator::alpha,
  symbol::SymbolId,
};

pub fn substitute(
  ctx: &mut Context,
  term: TermId,
  variable: SymbolId,
  replacement: TermId,
) -> TermId {
  match term {
    TermId::Variable(id) => substitute_variable(ctx, id, variable, replacement),
    TermId::Lambda(id) => substitute_lambda(ctx, id, variable, replacement),
    TermId::Apply(id) => substitute_apply(ctx, id, variable, replacement),
  }
}

fn substitute_variable(
  ctx: &mut Context,
  id: VariableId,
  variable: SymbolId,
  replacement: TermId,
) -> TermId {
  let current = ctx.terms.variables.get(id);
  if current.name == variable {
    replacement
  } else {
    let term = TermId::Variable(id);
    term
  }
}

fn substitute_apply(
  ctx: &mut Context,
  id: ApplyId,
  variable: SymbolId,
  replacement: TermId,
) -> TermId {
  let apply = *ctx.terms.applies.get(id);
  let function = substitute(ctx, apply.function, variable, replacement);
  let argument = substitute(ctx, apply.argument, variable, replacement);
  if function == apply.function && argument == apply.argument {
    let term = TermId::Apply(id);
    return term;
  }
  let new_apply = Apply { function, argument };
  let new_id = ctx.terms.add_apply(new_apply);
  new_id
}

fn substitute_lambda(
  ctx: &mut Context,
  id: LambdaId,
  variable: SymbolId,
  replacement: TermId,
) -> TermId {
  let lambda = *ctx.terms.lambdas.get(id);
  if lambda.parameter == variable {
    let term = TermId::Lambda(id);
    return term;
  }

  let captured = has_free_variable(ctx, replacement, lambda.parameter);
  if captured {
    let fresh = fresh_symbol(ctx, lambda.parameter, lambda.body, replacement);
    let renamed_body = alpha::rename(ctx, lambda.body, lambda.parameter, fresh);
    let body = substitute(ctx, renamed_body, variable, replacement);
    let new_lambda = Lambda { parameter: fresh, body, ..lambda };
    let new_id = ctx.terms.add_lambda(new_lambda);
    return new_id;
  }

  let body = substitute(ctx, lambda.body, variable, replacement);
  if body == lambda.body {
    let term = TermId::Lambda(id);
    return term;
  }

  let new_lambda = Lambda { body, ..lambda };
  let new_id = ctx.terms.add_lambda(new_lambda);
  new_id
}

pub fn has_free_variable(ctx: &Context, term: TermId, variable: SymbolId) -> bool {
  match term {
    TermId::Variable(id) => {
      let variable_entry = ctx.terms.variables.get(id);
      variable_entry.name == variable
    },
    TermId::Apply(id) => {
      let apply = ctx.terms.applies.get(id);
      let in_function = has_free_variable(ctx, apply.function, variable);
      if in_function {
        return true;
      }
      let in_argument = has_free_variable(ctx, apply.argument, variable);
      in_argument
    },
    TermId::Lambda(id) => {
      let lambda = ctx.terms.lambdas.get(id);
      if lambda.parameter == variable {
        return false;
      }
      let in_body = has_free_variable(ctx, lambda.body, variable);
      in_body
    },
  }
}

fn fresh_symbol(ctx: &mut Context, base: SymbolId, term1: TermId, term2: TermId) -> SymbolId {
  let base_name = ctx.symbols.resolve(base).to_string();
  let mut candidate = format!("{base_name}'");
  loop {
    let symbol = ctx.symbols.intern(&candidate);
    let free_in_first = has_free_variable(ctx, term1, symbol);
    let free_in_second = has_free_variable(ctx, term2, symbol);
    if !free_in_first && !free_in_second {
      return symbol;
    }
    candidate.push('\'');
  }
}
