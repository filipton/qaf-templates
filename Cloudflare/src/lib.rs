use worker::*;
mod router;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    return router::route(req, env).await;
}

//[[IF DATABASE Planetscale]]
use planetscale_driver::PSConnection;
pub fn get_db_conn(ctx: &RouteContext<()>) -> Result<PSConnection> {
    let host = ctx.secret("PS_HOST")?.to_string();
    let user = ctx.secret("PS_USER")?.to_string();
    let pass = ctx.secret("PS_PASS")?.to_string();

    return Ok(PSConnection::new(&host, &user, &pass));
}
//[[ENDIF]]
