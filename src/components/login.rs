use axum::{
    http::{header, HeaderMap}, response::IntoResponse, Form 
};
use maud::{html, Markup};
use axum_extra::extract::{cookie::Cookie};
use serde::Deserialize;
use uuid::Uuid;
use super::base::base;


use crate::database;

pub async fn validate_session_cookie(session: Option<&Cookie<'_>>) -> bool {
    if session.is_none() {return false;}
    let sessions = database::get_sessions().await;
    for token in sessions.iter(){
        if token == session.unwrap().value(){
            return true;
        }
    }
    false
}

pub async fn login(Form(userlogin): Form<UserLogin>) -> impl IntoResponse { // Changed return type
    println!("{:#?}", userlogin);
    let users = database::get_users().await;
    let user = users.iter().find(|user| {user.username == userlogin.username});
    let mut fail = false;
    if let Some(user) = user {
        if user.password != userlogin.password {
            fail = true;
        }
    }
    else {
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
    
    database::add_session_token(&token, &user.unwrap().id).await;

    let mut headers = HeaderMap::new();

    // Set a proper cookie with HttpOnly, Secure, SameSite=Lax for security
    let cookie = format!("session_cookie={token}; Path=/; HttpOnly; Secure; SameSite=Lax");
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());
    headers.insert("HX-Redirect", "/".parse().unwrap());

    (headers, html!{
        div class="text-green-500" {
            "Sucess"
        }
    }).into_response() // Return the headers and an empty body for the HTMX redirect
}

pub async fn login_page() -> Markup{
    base(html!{
        div class="flex h-screen"{
            div class="justify-center content-center p-8 m-auto w-full max-w-xs bg-gray-800 rounded-3xl shadow-2xl" {
                form hx-post="/login" hx-target="#response" hx-swap="outerHTML" class="flex flex-col"{
                    label for="username" class="block mb-1 text-sm font-medium text-gray-300"{
                        "username"
                    }
                    input id="username" type="text" name="username" autocomplete="username" required
                    class="py-2 px-2 mb-2 w-auto rounded-xl outline-rose-500 outline-2";
                    label for="password" class="block mt-4 mb-1 text-sm font-medium text-gray-300"{
                        "password"
                    }
                    input id="password" type="password" name="password" autocomplete="current-password" required
                    class="py-2 px-2 mb-2 w-auto rounded-xl outline-rose-500 outline-2";
                    button type="submit"
                    class="object-center py-4 mx-auto mt-8 w-full text-xl font-bold bg-rose-500 rounded-xl shadow-xl transition-all hover:shadow-2xl focus:w-full active:bg-gray-700 active:shadow-2xl bold hover:shadow-black active::shadow-black"{
                        "Login"
                    }
                }
                div id="response" {}
            }
        }
    })
}

#[derive(Deserialize, Debug)]
pub struct UserLogin{
    username: String,
    password: String
}
