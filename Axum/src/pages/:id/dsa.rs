use axum::response::Html;
use qaf_macros::{get, post};

#[get]
pub async fn test() -> Html<&'static str> {
    Html(
        r#"
        Hello
        "#,
    )
}

#[post]
pub async fn test2() -> Html<&'static str> {
    Html(
        r#"
        Hello2
        "#,
    )
}

#[post("siem")]
pub async fn test3() -> Html<&'static str> {
    Html(
        r#"
        Hello3
        "#,
    )
}
