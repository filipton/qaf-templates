use actix_web::{web::Data, App, HttpServer};
mod actix_scope;
use actix_scope::generated_scope;
use [[RUST_PROJECT_NAME]]::{AppState, StartupOptions};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let options = StartupOptions::new();

    //[[IF DATABASE Postgres(SQLX)]]
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(options.max_connections)
        .connect(&options.connection_string)
        .await
        .unwrap();

    let state = AppState { pool };
    //[[ENDIF]]
    
    //[[IF DATABASE Off]]
    let state = AppState {};
    //[[ENDIF]]

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
