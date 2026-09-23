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
        let result = Some(new_id);
        return Ok(result);
      }

      if let Some(argument) = reduce(apply.argument, ctx, options)? {
        let new_apply = Apply { argument, ..apply };
        let new_id = ctx.terms.add_apply(new_apply);
        let result = Some(new_id);
        return Ok(result);
      }

      if let Some(lambda_id) = apply.function.as_lambda() {
        let reduced = beta::reduce(ctx, lambda_id, apply.argument)?;
        let result = Some(reduced);
        return Ok(result);
      }

      let result = None;
      Ok(result)
    },
    TermKind::Lambda(_) | TermKind::Variable(_) => {
      let result = None;
      Ok(result)
    },
  }
}
