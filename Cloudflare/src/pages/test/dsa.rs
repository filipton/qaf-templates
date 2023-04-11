use qaf_macros::{get, post};
use worker::{Request, Response, Result, RouteContext};

#[get(":id")]
pub async fn route1(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!1")
}

#[post(":id/test")]
pub async fn route2(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!2")
}
