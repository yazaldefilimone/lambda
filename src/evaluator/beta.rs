use crate::{
  context::Context,
  core::{LambdaId, TermId},
  evaluator::substitute,
  result,
};

pub fn reduce(ctx: &mut Context, lambda_id: LambdaId, argument: TermId) -> result::Result<TermId> {
  let lambda = *ctx.terms.lambdas.get(lambda_id);
  let parameter = lambda.parameter;
  let body = lambda.body;
  let result = substitute::substitute(ctx, body, parameter, argument);
  Ok(result)
}
