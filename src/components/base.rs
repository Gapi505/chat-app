use maud::{html, Markup, DOCTYPE};
pub fn base(content: Markup) -> Markup{
    html!{
        (DOCTYPE)
        html lang="en-us"{
            head{
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title {
                    "Chat"
                }
                link href="/static/build.css" rel="stylesheet";
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" {}
            }
            body class="text-white bg-gray-900" {
                (content)
            }
        }
    }
}
