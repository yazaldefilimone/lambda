use crate::{
  context::Context,
  core::{Apply, Lambda, TermId, TermKind},
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

      let function_step = reduce(apply.function, ctx, options)?;
      if let Some(new_function) = function_step {
        let new_apply = Apply { function: new_function, argument: apply.argument };
        let new_id = ctx.terms.add_apply(new_apply);
        let result = Some(new_id);
        return Ok(result);
      }

      let argument_step = reduce(apply.argument, ctx, options)?;
      if let Some(new_argument) = argument_step {
        let new_apply = Apply { function: apply.function, argument: new_argument };
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
    TermKind::Lambda(id) => {
      if options.lazy {
        let result = None;
        return Ok(result);
      }

      let lambda = *ctx.terms.lambdas.get(id);
      let body_step = reduce(lambda.body, ctx, options)?;
      if let Some(new_body) = body_step {
        let new_lambda = Lambda { body: new_body };
        let new_id = ctx.terms.add_lambda(new_lambda);
        let result = Some(new_id);
        return Ok(result);
      }

      let result = None;
      Ok(result)
    },
    TermKind::Variable(_) => {
      let result = None;
      Ok(result)
    },
  }
}
