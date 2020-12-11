use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::time::SystemTime;
use std::env;
use telegram_notifyrs;

#[get("/hello")]
async fn hello() -> impl Responder {
    // Create or recreate the status file and give an error otherwise
    let _file = fs::File::create("status").expect("Couldn't create status file");

    // Show some text to the user, but since this wep app will be
    // called by a script, it doesn't matter what we write
    HttpResponse::Ok().body("Hello to you too!")
}

#[get("/check/{minutes}")]
async fn check(web::Path(minutes): web::Path<u64>) -> impl Responder {
    // Get some information about the file
    let metadata = fs::metadata("status").expect("Couldn't find status file");
    // Get the last modified time
    let modified_time = metadata.modified().expect("Couldn't get the last modified time for the status file");

    // Get the current time
    let now = SystemTime::now();

    // Calculate how much time has passed
    let difference = now.duration_since(modified_time).expect("Was the file created in the future?");

    // Convert the requested minutes into seconds
    let seconds = minutes * 60;
    // Convert the difference into seconds
    let difference = difference.as_secs();

    if difference < seconds {
        "It has said hello recently"
    } else if difference < seconds * 2 {
        let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
        let chat_id: i32 = env::var("TELEGRAM_CHAT_ID")
            .expect("Missing TELEGRAM_CHAT_ID environment variable")
            .parse()
            .expect("Error parsing TELEGRAM_CHAT_ID as i32");
        telegram_notifyrs::send_message("It is offline!".to_string(), &token, chat_id);

        "Offline and I should send a Telegram message"
    } else {
        "Still offline. Should I send a message?"
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
