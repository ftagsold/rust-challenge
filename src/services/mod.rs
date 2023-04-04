use crate::services::sum_file::sum_file;
use crate::services::sum_sentence::sum_sentence;
use crate::services::sum_vocals::sum_vocals;
use actix_web::{get, HttpResponse};
use tokio::{join, task};

pub mod sum_file;
pub mod sum_sentence;
pub mod sum_vocals;

#[get("/")]
pub async fn get_sums() -> HttpResponse {
    let task_sum_file = task::spawn(sum_file("./secret.txt"));
    let task_sum_vocals = task::spawn(sum_vocals("./secret.txt"));
    let task_sum_sentence = task::spawn(sum_sentence("./secret.txt"));

    let (sum_file, sum_vocals, sum_sentence) =
        join!(task_sum_file, task_sum_vocals, task_sum_sentence);

    HttpResponse::Ok().body(format!(
        "Sum file: {}\nSum vocals: {}\nSecret: {}",
        sum_file.unwrap(),
        sum_vocals.unwrap(),
        sum_sentence.unwrap()
    ))
}
