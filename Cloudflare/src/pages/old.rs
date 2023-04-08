use fnstack_cf_macro::route;
use worker::{Request, Response, Result, RouteContext};

#[route("route1")]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!")
}
