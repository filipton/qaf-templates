use fnstack_cf_macro::any;
use worker::{Request, Response, Result, RouteContext};

#[any]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok(format!("Hello, World! param: {}", ctx.param("id").unwrap()))
}
