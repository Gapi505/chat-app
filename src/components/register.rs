use axum::{http::HeaderMap, response::IntoResponse, Form};
use maud::{html, Markup};
use serde::Deserialize;
use crate::base;

pub async fn register(Form(register_user): Form<RegisterUser>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    if register_user.password != register_user.password_repeat{
        return (headers,
        html!{
                p class="text-rose-600 text-sm font-bold mt-1" id="response"{
                "passwords must match"
            }
        }
        ).into_response();
    }
    headers.insert("HX-Redirect", "/login".parse().unwrap());

    (headers, 
        html!{"sucess"}
    ).into_response()
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    username: String,
    password: String,
    password_repeat: String
}

pub async fn register_page() -> Markup{
    base(html!{
        div class="flex h-screen"{
            div class="justify-center content-center p-8 m-auto w-full max-w-xs bg-gray-800 rounded-3xl shadow-2xl" {
                form hx-post="/register" hx-target="#response" hx-swap="outerHTML" class="flex flex-col"{
                    label for="username" class="block mb-1 text-sm font-medium text-gray-300"{
                        "username"
                    }
                    input id="username" type="text" name="username" autocomplete="username" required
                    class="py-2 px-2 w-auto rounded-xl outline-rose-500 outline-2";
                    label for="password" class="block mt-4 mb-1 text-sm font-medium text-gray-300"{
                        "password"
                    }
                    input id="password" type="password" name="password" required
                    class="py-2 px-2 w-auto rounded-xl outline-rose-500 outline-2";
                    label for="password-repeat" class="block mt-4 mb-1 text-sm font-medium text-gray-300"{
                        "repeat password"
                    }
                    input id="password-repeat" type="password" name="password_repeat" required
                    class="py-2 px-2 mb-2 w-auto rounded-xl outline-rose-500 outline-2";
                    div id="response" {}
                    button type="submit"
                    class="object-center py-4 mx-auto mt-12 mb-2 w-full text-xl font-bold bg-rose-500 rounded-xl shadow-xl transition-all hover:shadow-2xl focus:w-full active:bg-gray-700 active:shadow-2xl bold hover:shadow-black active::shadow-black"{
                        "Register"
                    }
                }
                a href="/login"
                class ="text-sm text-rose-600 font-bold text-shadow-sm hover:text-rose-500 hover:text-shadow-lg hover:shadow-black flex-none" {
                    "already have an account?"
                }
            }
        }
    })
}
