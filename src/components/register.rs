use maud::{html, Markup};
use crate::base;

pub async fn register() {
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
                    input id="password-repeat" type="password" name="password-repeat" required
                    class="py-2 px-2 mb-2 w-auto rounded-xl outline-rose-500 outline-2";
                    button type="submit"
                    class="object-center py-4 mx-auto mt-12 mb-2 w-full text-xl font-bold bg-rose-500 rounded-xl shadow-xl transition-all hover:shadow-2xl focus:w-full active:bg-gray-700 active:shadow-2xl bold hover:shadow-black active::shadow-black"{
                        "Register"
                    }
                }
                a href="/login"
                class ="text-sm text-rose-600 font-bold text-shadow-sm hover:text-rose-500 hover:text-shadow-lg hover:shadow-black flex-none" {
                    "already have an account?"
                }
                div id="response" {}
            }
        }
    })
}
