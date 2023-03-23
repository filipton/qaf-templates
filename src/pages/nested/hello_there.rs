use actix_web::{get, HttpResponse, Responder};
use crate::AppState;

#[get("")]
pub async fn test() -> impl Responder {
    return HttpResponse::Ok().body("Hello from nested scope!");
}
