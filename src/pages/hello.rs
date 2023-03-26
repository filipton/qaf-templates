use actix_web::{get, post, web::Data, web::Path, HttpResponse, Responder};

use crate::AppState;

//[[IF DATABASE Postgres(SQLX)]]
#[derive(Debug, sqlx::FromRow)]
pub struct Test {
    pub id: i32,
    pub time: Option<i64>,
}
//[[ENDIF]]

#[get("/")]
pub async fn get_test(data: Data<AppState>) -> impl Responder {
    //[[IF DATABASE Postgres(SQLX)]]
    // SQLX EXAMPLE
    /*
       let mut conn = data.pool.acquire().await.unwrap();

       let rows = sqlx::query_as!(Test, "SELECT * FROM tests")
       .fetch_all(&mut conn)
       .await
       .unwrap();
       */
    //[[ENDIF]]

    return HttpResponse::Ok().body("It works!!!");
}

#[get("/name/{name}")]
pub async fn get_test_name(name: Path<String>) -> impl Responder {
    let name = name.into_inner();

    return HttpResponse::Ok().body(format!("Hello {}!", name));
}

//[[IF DATABASE Postgres(SQLX)]]
/*
#[actix_web::post("/")]
pub async fn post_test(data: Data<AppState>) -> impl Responder {
    let mut conn = data.pool.acquire().await.unwrap();

    let id = rand::random::<i32>();

    let row = sqlx::query_as!(Test, "INSERT INTO tests (id) VALUES ($1) RETURNING *", id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    return HttpResponse::Ok().body(format!("Hello {:?}!", row));
}
*/
//[[ENDIF]]

// THIS WONT BE ADDED TO THE ACTIX SCOPE
pub async fn get_post_indedsadsadx() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}
