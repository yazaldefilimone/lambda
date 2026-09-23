use crate::{
  context::Context,
  core::{Apply, TermId, TermKind},
  evaluator::{beta, strategy::StrategyOptions},
  result,
};

pub fn reduce(
  term: TermId,
  ctx: &mut Context,
  options: &StrategyOptions,
) -> result::Result<Option<TermId>> {
  match term.kind() {
    TermKind::Apply(id) => {
      let apply = *ctx.terms.applies.get(id);

      if let Some(function) = reduce(apply.function, ctx, options)? {
        let new_apply = Apply { function, ..apply };
        let new_id = ctx.terms.add_apply(new_apply);
        return Ok(Some(new_id));
      }

      if let Some(argument) = reduce(apply.argument, ctx, options)? {
        let new_apply = Apply { argument, ..apply };
        let new_id = ctx.terms.add_apply(new_apply);
        return Ok(Some(new_id));
      }

      if let Some(lambda_id) = apply.function.as_lambda() {
        let reduced = beta::reduce(ctx, lambda_id, apply.argument)?;
        return Ok(Some(reduced));
      }

      Ok(None)
    },
    TermKind::Lambda(_) | TermKind::Variable(_) => Ok(None),
  }
}
