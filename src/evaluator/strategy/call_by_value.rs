use crate::{
  context::Context,
  core::{Apply, TermId},
  evaluator::{beta, strategy::StrategyOptions},
  result,
};

pub fn reduce(
  term: TermId,
  ctx: &mut Context,
  options: &StrategyOptions,
) -> result::Result<Option<TermId>> {
  match term {
    TermId::Apply(id) => {
      let apply = *ctx.terms.applies.get(id);

      let function_step = reduce(apply.function, ctx, options)?;
      if let Some(new_function) = function_step {
        let new_apply = Apply { function: new_function, ..apply };
        let new_id = ctx.terms.add_apply(new_apply);
        let result = Some(new_id);
        return Ok(result);
      }

      let argument_step = reduce(apply.argument, ctx, options)?;
      if let Some(new_argument) = argument_step {
        let new_apply = Apply { argument: new_argument, ..apply };
        let new_id = ctx.terms.add_apply(new_apply);
        let result = Some(new_id);
        return Ok(result);
      }

      if let TermId::Lambda(lambda_id) = apply.function {
        let reduced = beta::reduce(ctx, lambda_id, apply.argument)?;
        let result = Some(reduced);
        return Ok(result);
      }

      let result = None;
      Ok(result)
    },
    TermId::Lambda(_) | TermId::Variable(_) => {
      let result = None;
      Ok(result)
    },
  }
}
