use actix_web::{web::Data, App, HttpServer};

//[[IF WEBSERVER Actix]]
mod actix_scope;
use actix_scope::generated_scope;
use rust_project_name_t::{AppState, StartupOptions};

//[[ENDIF]]

/*[[IF WEBSOCKET Tungstenite]]
mod websockets;
use std::{collections::HashMap, sync::Mutex};
use tokio::net::TcpListener;
use websockets::PeerMap;
[[ENDIF]]*/

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let options = StartupOptions::new();

    /*[[IF DATABASE Postgres(SQLX)]]
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(options.max_connections)
        .connect(&options.connection_string)
        .await
        .unwrap();

    let state = AppState { pool };
    [[ENDIF]]*/

    //[[IF DATABASE Off]]
    let state = AppState::default();
    //[[ENDIF]]

    /*[[IF WEBSOCKET Tungstenite]]
    let ws_state = PeerMap::new(Mutex::new(HashMap::new()));

    tokio::spawn(async move {
        let try_socket = TcpListener::bind(&options.websocket_bind_address).await;
        let listener = try_socket.expect("Failed to bind");
        println!(
            "Websockets listening on: {}",
            options.websocket_bind_address
        );

        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(websockets::handle_connection(
                ws_state.clone(),
                stream,
                addr,
            ));
        }
    });
    [[ENDIF]]*/

    println!("Starting server at {}", options.bind_address);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(generated_scope())
    })
    .bind(options.bind_address)?
    .run()
    .await
}
