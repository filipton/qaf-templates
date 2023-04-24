//[[IF WEBSOCKET On]]
use futures::StreamExt;
//[[ENDIF]]
use qaf_macros::{get, on};
use worker::{
    console_log, EventStream, Request, Response, Result, RouteContext, WebSocket, WebSocketPair,
    WebsocketEvent,
};

#[get]
pub async fn index(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!")
}

#[get("fetch")]
pub async fn fetch(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = reqwest::get("https://gist.githubusercontent.com/filipton/f2c350b40b56896abbc5b010e231dc52/raw/c0284276e19ee1ed5074804e12a01244611279c6/blockurls.txt")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Response::ok(format!("body: {}", body))
}

//[[IF DATABASE Planetscale]]
// You must set PS_HOST, PS_USER, PS_PASS secrets in your .dev.vars file!
#[get("db")]
pub async fn db(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let mut conn = crate::get_db_conn(&ctx)?;
    let res: String = planetscale_driver::query("SELECT 'Test 123'")
        .fetch_scalar(&mut conn)
        .await
        .unwrap();

    Response::ok(format!("Test string: {}", res))
}

//[[IF WEBSOCKET On]]
#[on("ws")]
pub async fn ws(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let upgrade_header = req.headers().get("Upgrade");
    if !upgrade_header.is_ok() || upgrade_header.unwrap() != Some("websocket".to_string()) {
        return Ok(Response::ok("Websocket Error").unwrap().with_status(426));
    }

    let web_socket_pair = WebSocketPair::new()?;
    let client = web_socket_pair.client;
    let server = web_socket_pair.server;
    websocket(server).await?;

    return Ok(Response::from_websocket(client).unwrap().with_status(101));
}

async fn websocket(ws: WebSocket) -> Result<()> {
    ws.accept()?;
    ws.send(&"Connected!".to_string())?;

    wasm_bindgen_futures::spawn_local(async move {
        let mut event_stream: EventStream = ws.events().expect("Failed to get event stream");
        while let Some(event) = event_stream.next().await {
            match event.expect("Failed to get event") {
                WebsocketEvent::Message(msg) => {
                    if let Some(text) = msg.text() {
                        ws.send_with_str(text).expect("Failed to send message");
                    }
                }
                _ => {}
            }
        }
    });
    Ok(())
}
//[[ENDIF]]
