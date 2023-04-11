use anyhow::Result;
use axum::Router;
use rust_project_name_t::{AppState, StartupOptions};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod router;

/*[[IF WEBSOCKET Tungstenite]]
mod websockets;
use std::{collections::HashMap, sync::Mutex};
use tokio::net::TcpListener;
use websockets::PeerMap;
[[ENDIF]]*/

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let options = StartupOptions::new();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_project_name_t=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        /*[[IF DATABASE Postgres(SQLX)]]
        pool: sqlx::postgres::PgPoolOptions::new()
            .max_connections(options.max_connections)
            .connect(&options.connection_string)
            .await?,
        [[ENDIF]]*/
    };

    /*[[IF WEBSOCKET Tungstenite]]
    let ws_state = PeerMap::new(Mutex::new(HashMap::new()));

    tokio::spawn(async move {
        let try_socket = TcpListener::bind(&options.websocket_bind_address).await;
        let listener = try_socket.expect("Failed to bind");
        tracing::debug!("websockets listening on {}", options.websocket_bind_address);

        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(websockets::handle_connection(
                ws_state.clone(),
                stream,
                addr,
            ));
        }
    });
    [[ENDIF]]*/

    let app = Router::new()
        .with_state(state.clone())
        .merge(router::router(state).await)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = options.bind_address.parse::<SocketAddr>()?;
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
