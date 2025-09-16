use axum::{
    http::{StatusCode}, response::IntoResponse, routing::{get, post}, Router
};
use maud::html;
use axum_extra::extract::CookieJar;

mod database;
mod components;

use components::base::base;
use components::login;
use components::register;
use components::interface;


#[tokio::main]
async fn main() {
    database::create_db().await;
    let stat = tower_http::services::ServeDir::new("static");
    let app = Router::new()
        .route("/", get(interface::skeleton))
        .route("/login", get(login::login_page))
        .route("/login", post(login::login))
        .route("/register", get(register::register_page))
        .route("/register", post(register::register))
        .nest_service("/static", stat);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn index(jar: CookieJar) -> impl IntoResponse {
    bail_if_unauthenticated!(jar);
    let user = format!{"{:#?}", login::get_user_from_cookie(jar).await};
    (StatusCode::OK,
        base(html!{
            h1 {
                "Hello world"
            }
            pre {(user)}
        })
    ).into_response()
}
