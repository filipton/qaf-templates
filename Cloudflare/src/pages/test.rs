use fnstack_cf_macro::route;
use worker::{Context, Env, Request, Response, Result};

#[route("route1")]
pub async fn route1(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok("Hello, World!")
}
