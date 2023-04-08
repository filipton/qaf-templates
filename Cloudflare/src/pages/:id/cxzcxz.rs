use fnstack_cf_macro::route;
use worker::{Request, Response, Result, RouteContext};

#[route]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok(format!("Hello, World! param: {}", ctx.param("id").unwrap()))
}
