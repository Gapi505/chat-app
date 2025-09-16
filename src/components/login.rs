use super::base::base;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{
    http::{header, HeaderMap},
    response::IntoResponse,
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use maud::{html, Markup};
use serde::Deserialize;
use uuid::Uuid;

use crate::database;
use crate::database::DbUser;

pub async fn validate_session_cookie(session: Option<&Cookie<'_>>) -> bool {
    if session.is_none() {
        return false;
    }
    /*let sessions = database::get_sessions().await;
    for token in sessions.iter() {
        if token == session.unwrap().value() {
            return true;
        }
    }*/
    database::get_session(session.unwrap().value()).await.is_some()
}

pub async fn get_user_from_cookie(jar: CookieJar) -> Option<DbUser>{
    let session = jar.get("session_cookie")?.value();
    database::get_user_by_token(session).await
}

#[macro_export]
macro_rules! bail_if_unauthenticated {
    ( $cookie_jar:ident ) => {
        let session_cookie = $cookie_jar.get("session_cookie");
        let is_valid = $crate::login::validate_session_cookie(session_cookie).await;
        if !is_valid {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::SET_COOKIE,
                "session_cookie=; Max-Age=0; Path=/; HttpOnly; Secure; SameSite=Lax"
                    .parse()
                    .unwrap(),
            );
            return (headers, axum::response::Redirect::to("/login")).into_response();
        }
    };
}

fn check_password(pass: &String, hash: &str) -> bool {
    let hash_parsed = match PasswordHash::new(hash) {
        Ok(hp) => hp,
        Err(e) => {
            eprintln!("error during pasword hash: {e}");
            return false;
        }
    };
    Argon2::default()
        .verify_password(pass.as_bytes(), &hash_parsed)
        .is_ok()
}

pub async fn login(Form(userlogin): Form<UserLogin>) -> impl IntoResponse {
    // Changed return type
    let mut headers = HeaderMap::new();
    let user_option = database::get_user_by_username(&userlogin.username).await;
    let auth_sucessful = if let Some(user) = &user_option {
        check_password(&userlogin.password, &user.password_hash)
    } else {
        let dummy_hash = "$argon2id$v=19$m=19456,t=2,p=1$notW033GFLwVy6He+C/LIw$a4RZG+aYqakWbkxDITd1d1j22BxP+Xwc+sF//d72EJY";
        check_password(&userlogin.password, dummy_hash)
    };
    if !auth_sucessful {
        return (
            headers,
            html! {
                div class="text-rose-600 text-sm font-bold mt-1" id="response"{
                    "Invalid credentials, you fool!"
                }
            }, // Return the HTML as a string or implement IntoResponse for Markup
        )
            .into_response();
    }
    let token = Uuid::new_v4().hyphenated().to_string();

    database::add_session_token(&token, &user_option.unwrap().id).await;

    let week_in_secconds:u64 = 7*24*60*60;

    // Set a proper cookie with HttpOnly, Secure, SameSite=Lax for security
    let cookie = format!("session_cookie={token}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={week_in_secconds}");
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());
    headers.insert("HX-Redirect", "/".parse().unwrap());

    (
        headers,
        html! {
            div class="text-green-500" {
                "Sucess"
            }
        },
    )
        .into_response() // Return the headers and an empty body for the HTMX redirect
}

pub async fn login_page() -> Markup {
    base(html! {
        div class="flex h-screen"{
            div class="justify-center content-center p-8 m-auto w-full max-w-xs bg-gray-800 rounded-3xl shadow-2xl" {
                form hx-post="/login" hx-target="#response" hx-swap="outerHTML" hx-on::after-request="document.getElementById('password').value = ''" class="flex flex-col"{
                    label for="username" class="block mb-1 text-sm font-medium text-gray-300"{
                        "username"
                    }
                    input id="username" type="text" name="username" autocomplete="username" required
                    class="py-2 px-2 w-auto rounded-xl outline-rose-500 outline-2";
                    label for="password" class="block mt-4 mb-1 text-sm font-medium text-gray-300"{
                        "password"
                    }
                    input id="password" type="password" name="password" autocomplete="current-password" required
                    class="py-2 px-2 w-auto rounded-xl outline-rose-500 outline-2";
                    div id="response" {}
                    button type="submit"
                    class="object-center py-4 mx-auto mt-12 mb-2 w-full text-xl font-bold bg-rose-500 rounded-xl shadow-xl transition-all hover:shadow-2xl focus:w-full active:bg-gray-700 active:shadow-2xl bold hover:shadow-black active::shadow-black"{
                        "Login"
                    }
                }
                a href="/register"
                class ="text-sm text-rose-600 font-bold text-shadow-sm hover:text-rose-500 hover:text-shadow-lg hover:shadow-black flex-none" {
                    "dont have an account?"
                }
            }
        }
    })
}

#[derive(Deserialize, Debug)]
pub struct UserLogin {
    username: String,
    password: String,
}
