use crate::{
  context::Context,
  core::{Apply, ApplyId, Lambda, LambdaId, TermId, TermKind, Variable, VariableId},
};

pub fn shift(ctx: &mut Context, delta: i32, cutoff: u32, term: TermId) -> TermId {
  if delta == 0 {
    return term;
  }
  match term.kind() {
    TermKind::Variable(id) => shift_variable(ctx, term, id, delta, cutoff),
    TermKind::Lambda(id) => shift_lambda(ctx, term, id, delta, cutoff),
    TermKind::Apply(id) => shift_apply(ctx, term, id, delta, cutoff),
  }
}

fn shift_variable(
  ctx: &mut Context,
  term: TermId,
  id: VariableId,
  delta: i32,
  cutoff: u32,
) -> TermId {
  let var = *ctx.terms.variables.get(id);
  if var.index < cutoff {
    term
  } else {
    let new_index = (var.index as i32 + delta) as u32;
    let new_var = Variable { index: new_index };
    ctx.terms.add_variable(new_var)
  }
}

fn shift_lambda(ctx: &mut Context, term: TermId, id: LambdaId, delta: i32, cutoff: u32) -> TermId {
  let lambda = *ctx.terms.lambdas.get(id);
  let new_body = shift(ctx, delta, cutoff + 1, lambda.body);
  if new_body == lambda.body {
    term
  } else {
    let new_lambda = Lambda { body: new_body };
    ctx.terms.add_lambda(new_lambda)
  }
}

fn shift_apply(ctx: &mut Context, term: TermId, id: ApplyId, delta: i32, cutoff: u32) -> TermId {
  let apply = *ctx.terms.applies.get(id);
  let function = shift(ctx, delta, cutoff, apply.function);
  let argument = shift(ctx, delta, cutoff, apply.argument);
  if function == apply.function && argument == apply.argument {
    term
  } else {
    let new_apply = Apply { function, argument };
    ctx.terms.add_apply(new_apply)
  }
}

pub fn substitute(ctx: &mut Context, cutoff: u32, value: TermId, term: TermId) -> TermId {
  match term.kind() {
    TermKind::Variable(id) => substitute_variable(ctx, term, id, cutoff, value),
    TermKind::Lambda(id) => substitute_lambda(ctx, term, id, cutoff, value),
    TermKind::Apply(id) => substitute_apply(ctx, term, id, cutoff, value),
  }
}

fn substitute_variable(
  ctx: &mut Context,
  term: TermId,
  id: VariableId,
  cutoff: u32,
  value: TermId,
) -> TermId {
  let var = *ctx.terms.variables.get(id);
  if var.index == cutoff {
    shift(ctx, cutoff as i32, 0, value)
  } else if var.index > cutoff {
    let new_var = Variable { index: var.index - 1 };
    ctx.terms.add_variable(new_var)
  } else {
    term
  }
}

fn substitute_lambda(
  ctx: &mut Context,
  term: TermId,
  id: LambdaId,
  cutoff: u32,
  value: TermId,
) -> TermId {
  let lambda = *ctx.terms.lambdas.get(id);
  let new_body = substitute(ctx, cutoff + 1, value, lambda.body);
  if new_body == lambda.body {
    term
  } else {
    let new_lambda = Lambda { body: new_body };
    ctx.terms.add_lambda(new_lambda)
  }
}

fn substitute_apply(
  ctx: &mut Context,
  term: TermId,
  id: ApplyId,
  cutoff: u32,
  value: TermId,
) -> TermId {
  let apply = *ctx.terms.applies.get(id);
  let function = substitute(ctx, cutoff, value, apply.function);
  let argument = substitute(ctx, cutoff, value, apply.argument);
  if function == apply.function && argument == apply.argument {
    term
  } else {
    let new_apply = Apply { function, argument };
    ctx.terms.add_apply(new_apply)
  }
}
