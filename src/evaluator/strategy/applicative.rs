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

      if let Some(function) = reduce(apply.function, ctx, options)? {
        let new_apply = Apply { function, argument: apply.argument };
        let new_id = ctx.terms.add_apply(new_apply);
        return Ok(Some(new_id));
      }

      if let Some(argument) = reduce(apply.argument, ctx, options)? {
        let new_apply = Apply { function: apply.function, argument };
        let new_id = ctx.terms.add_apply(new_apply);
        return Ok(Some(new_id));
      }

      if let Some(lambda_id) = apply.function.as_lambda() {
        let reduced = beta::reduce(ctx, lambda_id, apply.argument)?;
        return Ok(Some(reduced));
      }

      Ok(None)
    },
    TermKind::Lambda(id) => {
      if options.lazy {
        return Ok(None);
      }

      if ctx.options.trace {
        ctx.current_scope.push(id);
      }

      let lambda = *ctx.terms.lambdas.get(id);
      let body_step = reduce(lambda.body, ctx, options)?;

      if ctx.options.trace {
        ctx.current_scope.pop();
      }

      if let Some(new_body) = body_step {
        let new_lambda = Lambda { body: new_body };
        let parameter = ctx.terms.parameter(id);
        let annotation = ctx.terms.annotation(id);
        let new_id = ctx.terms.add_annotated(new_lambda, parameter, annotation);
        return Ok(Some(new_id));
      }

      Ok(None)
    },
    TermKind::Variable(_) => Ok(None),
  }
}
