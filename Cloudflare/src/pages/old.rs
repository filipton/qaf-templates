use fnstack_cf_macro::on;
use worker::{Request, Response, Result, RouteContext};

#[on("route1")]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!")
}
