// use std::{fs::File, io::Read};

use axum::{extract::Path, response::Html};

#[axum::debug_handler]
pub async fn meow(Path(num): Path<u8>) -> axum::response::Html<String> {
    println!("data: '{}'", num);
    Html(
        format!(
            "
    <button hx-get=\"/meow/{}\" hx-swap=\"outerHTML\" hx-target=\"this\">
    Hi {}
    </button>
    ",
            num + 1,
            ":3 ".repeat(num as usize).trim_end()
        )
        .to_owned(),
    )
}

// don't know how to copy this along with build
// pub async fn home() -> axum::response::Html<String> {
//     let mut buf = String::new();
//     let mut file = File::open("./index.html").expect("index.html does not exist");
//     let result = Read::read_to_string(&mut file, &mut buf);
//     match result {
//         Ok(_) => Html(buf),
//         Err(e) => Html(format!("Error occurred while reading file: {e}").to_owned()),
//     }
// }

pub async fn home() -> axum::response::Html<String> {
    Html(homepage())
}

fn homepage() -> String {
    "<!DOCTYPE html>
<html>
  <head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <script src=\"https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js\" integrity=\"sha384-/TgkGk7p307TH7EXJDuUlgG3Ce1UVolAOFopFekQkkXihi5u/6OCvVKyz1W+idaz\" crossorigin=\"anonymous\"></script>
  </head>
  <body>
    <main>
    <button hx-get=\"/meow/2\" hx-swap=\"outerHTML\" hx-target=\"this\">
    Hi :3
    </button>
    </main>
  </body>
</html>"
        .to_owned()
}
