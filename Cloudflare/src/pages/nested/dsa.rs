use fnstack_cf_macro::any;
use worker::{Request, Response, Result, RouteContext};

#[any]
pub async fn hw_empty(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, Void!!")
}

#[any("siem")]
pub async fn siema(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Siema")
}
