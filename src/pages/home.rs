// use std::{fs::File, io::Read};

use axum::{extract::Path, response::Html};

#[axum::debug_handler]
pub async fn meow(Path(num): Path<u64>) -> axum::response::Html<String> {
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
    Html(homepage().to_owned())
}

const fn homepage() -> &'static str {
    include_str!("index.html")
}
