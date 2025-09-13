use axum::{
    http::{header, HeaderMap, HeaderName, StatusCode}, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router
};
use maud::{html, Markup, DOCTYPE};
use axum_extra::extract::{cookie::Cookie, CookieJar};

mod database;
mod components;

use components::base::base;
use components::login;


#[tokio::main]
async fn main() {
    database::create_db().await;
    let stat = tower_http::services::ServeDir::new("static");
    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login::login_page))
        .route("/login", post(login::login))
        .nest_service("/static", stat);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn index(jar: CookieJar) -> impl IntoResponse {
    let session = jar.get("session_cookie");
    if !login::validate_session_cookie(session).await{
        return Redirect::to("/login").into_response();
    }
    (StatusCode::OK,
        base(html!{
            h1 {
                "Hello world"
            }
        })
    ).into_response()
}
