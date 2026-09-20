use crate::{
  context::Context,
  core::{Apply, ApplyId, Lambda, LambdaId, TermId, Variable, VariableId},
  symbol::SymbolId,
};

pub fn rename(ctx: &mut Context, term: TermId, from: SymbolId, to: SymbolId) -> TermId {
  match term {
    TermId::Variable(id) => rename_variable(ctx, id, from, to),
    TermId::Apply(id) => rename_apply(ctx, id, from, to),
    TermId::Lambda(id) => rename_lambda(ctx, id, from, to),
  }
}

fn rename_variable(ctx: &mut Context, id: VariableId, from: SymbolId, to: SymbolId) -> TermId {
  let variable = ctx.terms.variables.get(id);
  if variable.name == from {
    let new_variable = Variable { name: to };
    let new_id = ctx.terms.add_variable(new_variable);
    new_id
  } else {
    let term = TermId::Variable(id);
    term
  }
}

fn rename_apply(ctx: &mut Context, id: ApplyId, from: SymbolId, to: SymbolId) -> TermId {
  let apply = *ctx.terms.applies.get(id);
  let function = rename(ctx, apply.function, from, to);
  let argument = rename(ctx, apply.argument, from, to);
  if function == apply.function && argument == apply.argument {
    let term = TermId::Apply(id);
    return term;
  }
  let new_apply = Apply { function, argument };
  let new_id = ctx.terms.add_apply(new_apply);
  new_id
}

fn rename_lambda(ctx: &mut Context, id: LambdaId, from: SymbolId, to: SymbolId) -> TermId {
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
    let term = TermId::Lambda(id);
    return term;
  }
  let new_lambda = Lambda { body, ..lambda };
  let new_id = ctx.terms.add_lambda(new_lambda);
  new_id
}
