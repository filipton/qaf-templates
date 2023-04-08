use fnstack_cf_macro::route;
use worker::{Request, Response, Result, RouteContext};

#[route]
pub async fn hw_empty(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, Void!!")
}

#[route("siem")]
pub async fn siema(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Siema")
}
