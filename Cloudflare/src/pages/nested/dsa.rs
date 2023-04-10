use fnstack_cf_macro::on;
use worker::{Request, Response, Result, RouteContext};

#[on]
pub async fn hw_empty(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, Void!!")
}

#[on("siem")]
pub async fn siema(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Siema")
}
