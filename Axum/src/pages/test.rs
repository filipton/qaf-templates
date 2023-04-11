use axum::{extract::State, response::Html};
use qaf_macros::get;
use rust_project_name_t::AppState;

#[get]
pub async fn test(State(data): State<AppState>) -> Html<String> {
    let title = &data.test;

    Html(format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>{}</title>
            </head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <div>
                        <label>
                            Upload file:
                            <input type="file" name="file" multiple>
                        </label>
                    </div>

                    <div>
                        <input type="submit" value="Upload files">
                    </div>
                </form>
            </body>
        </html>
        "#,
        title
    ))
}
