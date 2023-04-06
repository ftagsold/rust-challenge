use actix_web::{get, HttpResponse};

use crate::services::parser::parse;

pub mod parser;

#[get("/")]
pub async fn get_sums() -> HttpResponse {
    HttpResponse::Ok().json(parse("./secret.txt"))
}
