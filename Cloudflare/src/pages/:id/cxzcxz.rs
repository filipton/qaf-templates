use fnstack_cf_macro::on;
use worker::{Request, Response, Result, RouteContext};

#[on]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok(format!("Hello, World! param: {}", ctx.param("id").unwrap()))
}
