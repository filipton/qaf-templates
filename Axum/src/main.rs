use anyhow::Result;
use axum::Router;
use rust_project_name_t::{AppState, StartupOptions};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod router;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let options = StartupOptions::new();

    let state = AppState {
        /*[[IF DATABASE Postgres(SQLX)]]
        pool: sqlx::postgres::PgPoolOptions::new()
            .max_connections(options.max_connections)
            .connect(&options.connection_string)
            .await?,
        [[ENDIF]]*/
        test: "test".to_string(),
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_project_name_t=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(router::router(state).await)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = options.bind_address.parse::<SocketAddr>()?;
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
