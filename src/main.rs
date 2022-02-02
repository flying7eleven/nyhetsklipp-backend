#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
struct Selectors {
    title: String,
    full_text_link: String,
}

#[derive(Deserialize)]
struct ArticleSourceParameters<'r> {
    source: &'r str,
    selectors: Selectors,
}

#[get("/live/articles/for", data = "<article_source>")]
fn index(article_source: Json<ArticleSourceParameters<'_>>) -> String {
    article_source.0.source.to_string()
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
