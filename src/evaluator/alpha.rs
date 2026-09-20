use crate::{
  context::Context,
  core::{Apply, ApplyId, Lambda, LambdaId, TermId, TermKind, Variable, VariableId},
  symbol::SymbolId,
};

pub fn rename(ctx: &mut Context, term: TermId, from: SymbolId, to: SymbolId) -> TermId {
  if term.is_closed() {
    return term;
  }
  match term.kind() {
    TermKind::Variable(id) => rename_variable(ctx, term, id, from, to),
    TermKind::Apply(id) => rename_apply(ctx, term, id, from, to),
    TermKind::Lambda(id) => rename_lambda(ctx, term, id, from, to),
  }
}

fn rename_variable(ctx: &mut Context, term: TermId, id: VariableId, from: SymbolId, to: SymbolId) -> TermId {
  let variable = ctx.terms.variables.get(id);
  if variable.name == from {
    let new_variable = Variable { name: to };
    let new_id = ctx.terms.add_variable(new_variable);
    new_id
  } else {
    term
  }
}

fn rename_apply(ctx: &mut Context, term: TermId, id: ApplyId, from: SymbolId, to: SymbolId) -> TermId {
  let apply = *ctx.terms.applies.get(id);
  let function = rename(ctx, apply.function, from, to);
  let argument = rename(ctx, apply.argument, from, to);
  if function == apply.function && argument == apply.argument {
    return term;
  }
  let new_apply = Apply { function, argument };
  let new_id = ctx.terms.add_apply(new_apply);
  new_id
}

fn rename_lambda(ctx: &mut Context, term: TermId, id: LambdaId, from: SymbolId, to: SymbolId) -> TermId {
  let lambda = *ctx.terms.lambdas.get(id);
  if lambda.parameter == from {
    let parameter = to;
    let body = rename(ctx, lambda.body, from, to);
    let new_lambda = Lambda { parameter, body, ..lambda };
    let new_id = ctx.terms.add_lambda(new_lambda);
    return new_id;
  }
  let body = rename(ctx, lambda.body, from, to);
  if body == lambda.body {
    return term;
  }
  let new_lambda = Lambda { body, ..lambda };
  let new_id = ctx.terms.add_lambda(new_lambda);
  new_id
}
