use fnstack_cf_macro::any;
use worker::{Request, Response, Result, RouteContext};

#[any("route1")]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!")
}
