use worker::*;
mod router;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    return router::router(req, env).await;
}
