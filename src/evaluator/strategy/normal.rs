use crate::{
  context::Context,
  core::{Apply, Lambda, TermId},
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
      match apply.function {
        TermId::Lambda(lambda_id) => {
          let reduced = beta::reduce(ctx, lambda_id, apply.argument)?;
          let result = Some(reduced);
          Ok(result)
        },
        _ => {
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

          let result = None;
          Ok(result)
        },
      }
    },
    TermId::Lambda(id) => {
      if options.lazy {
        let result = None;
        return Ok(result);
      }

      let lambda = *ctx.terms.lambdas.get(id);
      let body_step = reduce(lambda.body, ctx, options)?;
      if let Some(new_body) = body_step {
        let new_lambda = Lambda { body: new_body, ..lambda };
        let new_id = ctx.terms.add_lambda(new_lambda);
        let result = Some(new_id);
        return Ok(result);
      }

      let result = None;
      Ok(result)
    },
    TermId::Variable(_) => {
      let result = None;
      Ok(result)
    },
  }
}
