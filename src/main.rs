use axum::{
    http::{header, HeaderMap, HeaderName, StatusCode}, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router
};
use maud::{html, Markup, DOCTYPE};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Serialize, Deserialize};
use sqlx::{migrate::MigrateDatabase, prelude::FromRow, Sqlite, SqlitePool, Row};
use uuid::Uuid;



const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    create_db().await;
    let stat = tower_http::services::ServeDir::new("static");
    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login_page))
        .route("/login", post(login))

        .nest_service("/static", stat);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let _result = sqlx::query("
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(32) NOT NULL
);
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    session_token STRING NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
").execute(&db).await.unwrap();
}

async fn index(jar: CookieJar) -> impl IntoResponse {
    let session = jar.get("session_cookie");
    if !validate_session_cookie(session).await{
        return Redirect::to("/login").into_response();
    }
    (StatusCode::OK, html!{
        (DOCTYPE)
        html lang="en-us"{
            head{
                meta charset="UTF-8";
                title {
                    "Chat"
                }
                link href="/static/build.css" rel="stylesheet";
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" {}
            }
            body class="bg-gray-900 text-white" {
                h1 {
                    "Hello world"
                }
            }
        }
    }).into_response()
}

async fn validate_session_cookie<'a>(session: Option<&'a Cookie<'_>>) -> bool {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let sessions = sqlx::query("select session_token from sessions").fetch_all(&db).await.unwrap();
    if session.is_none() {return false;}
    for ses in sessions.iter(){
        let token = ses.get::<String, &str>("session_token");
        println!("{token}");
        if token == session.unwrap().value(){
            return true;
        }
    }
    return false;

}

async fn login(Form(userlogin): Form<UserLogin>) -> impl IntoResponse { // Changed return type
    println!("{:#?}", userlogin);
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let user_results = sqlx::query_as::<_, DbUser>("SELECT * from users").fetch_all(&db).await.unwrap();
    let user = user_results.iter().find(|user| {user.username == userlogin.username});
    let mut fail = false;
    if user.is_none() { fail = true}
    else if user.unwrap().password != userlogin.password {
        fail = true;
    }
    if fail{
        return (
            axum::http::StatusCode::UNAUTHORIZED,
            html! {
                div class="text-red-500" id="response"{
                    "Invalid credentials, you fool!"
                }
            }.into_string() // Return the HTML as a string or implement IntoResponse for Markup
        ).into_response();
    }
    let token = Uuid::new_v4().hyphenated().to_string();
    println!("{token}");

    let session = token; // In a real app, this would be a real session ID/token

    sqlx::query("insert into sessions (session_token, user_id) values (?, ?)").bind(&session).bind(user.unwrap().id).execute(&db).await.unwrap();
    let mut headers = HeaderMap::new();

    // Set a proper cookie with HttpOnly, Secure, SameSite=Lax for security
    let cookie = format!("session_cookie={session}; Path=/; HttpOnly; Secure; SameSite=Lax");
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());
    headers.insert("HX-Redirect", "/".parse().unwrap());

    // Return the headers. Axum will automatically create an empty body for this.
    // The `IntoResponse` trait handles converting HeaderMap into a response.
    (headers, html!{
        div class="text-green-500" {
            "Sucess"
        }
    }).into_response() // Return the headers and an empty body for the HTMX redirect
}

#[derive(Clone, FromRow, Debug)]
struct DbUser{
    id: i64,
    username: String,
    password: String
}


async fn login_page() -> Markup{
    html!{
        (DOCTYPE)
        html lang="en-us"{
            head{
                meta charset="UTF-8";
                title {
                    "Chat"
                }
                link href="/static/build.css" rel="stylesheet";
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js"{}
            }
            body class="bg-gray-900 text-white" {
                div class="flex h-screen"{
                    div class="bg-gray-800 p-16 m-auto rounded-3xl shadow-2xl justify-center content-center" {
                        form hx-post="/login" hx-target="#response" hx-swap="outerHTML" class="flex flex-col"{
                            input name="username" class="bg-gray-700 px-8 py-4 rounded-xl my-2";
                            input type="password" name="password" class="bg-gray-700 px-8 py-4 rounded-xl my-2";
                            button type="submit"
                            class="bg-gray-900 py-4 rounded-xl mt-8 w-2/3 object-center mx-auto hover:w-full transition-all hover:bg-gray-700
                            shadow-xl hover:shadow-2xl hover:shadow-black"{
                                "Login"
                            }
                        }
                        div id="response" {}
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct UserLogin{
    username: String,
    password: String
}
