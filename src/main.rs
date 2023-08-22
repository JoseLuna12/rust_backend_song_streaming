use actix_files::Files;
// use actix_files::Files;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use db::mockdb::InitApp;

use crate::db::models::ErrorResponse;
mod db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/{username}")]
async fn get_user(username: web::Path<String>) -> HttpResponse {
    let init_app_values = init_app_data();

    let user = init_app_values.get_user_by_username(username.to_string());

    match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => {
            let error_msg = ErrorResponse {
                code: "404".to_owned(),
                message: format!("User: {} not found", { username.to_string() }),
            };
            HttpResponse::Forbidden().json(error_msg)
        }
    }
}

#[get("/user")]
async fn get_users() -> HttpResponse {
    let init_app_values = init_app_data();

    let users = init_app_values.users;

    HttpResponse::Ok().json(users)
}

#[get("/song/{songid}")]
async fn get_song(songid: web::Path<String>) -> HttpResponse {
    let init_app_values = init_app_data();

    let song = init_app_values.get_song_by_id(songid.to_string());

    match song {
        Some(s) => HttpResponse::Ok().json(s),
        None => HttpResponse::NotFound().body("not found"),
    }
}

#[get("/play/{songid}")]
async fn play_song(req: HttpRequest, songid: web::Path<String>) -> HttpResponse {
    let init_app_values = init_app_data();

    let song = init_app_values.get_song_by_id(songid.to_string());

    match song {
        Some(s) => {
            let file_path = format!("./static/{}", s.location);
            let file = actix_files::NamedFile::open_async(file_path).await.unwrap();
            file.into_response(&req)
        }
        None => HttpResponse::NotFound().body("not found"),
    }
}

#[get("/song")]
async fn get_songs() -> HttpResponse {
    let init_app_values = init_app_data();
    HttpResponse::Ok().json(init_app_values.music)
}

fn init_app_data() -> InitApp<'static> {
    db::mockdb::InitApp::init()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", ".").show_files_listing())
            .service(hello)
            .service(get_songs)
            .service(get_song)
            .service(play_song)
            .service(get_user)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
