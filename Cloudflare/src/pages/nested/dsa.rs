use fnstack_cf_macro::route;
use worker::{Context, Env, Request, Response, Result};

#[route]
pub async fn hw_empty(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok("Hello, Void!!")
}


#[route("siem")]
pub async fn siema(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok("Siema")
}
