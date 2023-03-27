#![allow(unused)]
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::time::Duration;
use std::{env, fs};
pub mod auth;
pub mod challenge;
pub mod info;
pub mod misc;
pub mod submit;
use std::{thread, time};

const SERVER_NUM: i64 = 3;

#[actix_web::main]
async fn main() {
    if let Err(err) = env::var("JWT_KEY") {
        panic!("JWT_KEY is not set");
    }
    if let Err(err) = env::var("DATABASE_URL") {
        panic!("DATABASE_URL is not set");
    }
    if let Err(err) = env::var("CONTEST_START") {
        panic!("CONTEST_START is not set");
    }
    if let Err(err) = env::var("CONTEST_END") {
        panic!("CONTEST_END is not set");
    }
    if let Err(err) = env::var("SERVER_NUM") {
        panic!("SERVER_NUM is not set");
    }
    if let Ok(s) = env::var("LOCAL") {
        HttpServer::new(|| {
            let cors = Cors::default()
                .allowed_origin_fn(|origin, _req_head| true)
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec!["content-type", "access-control-allow-origin"]);

            App::new()
                .wrap(cors)
                .service(auth::auth::login)
                .service(submit::submit::submit)
                .service(challenge::challenge::challenge)
                .service(challenge::challenge::vote)
                .service(info::info::submission_info)
                .service(info::info::submissions_list)
                .service(info::info::challenge_info)
                .service(info::info::challenges_list)
                .service(info::info::users)
                .service(info::info::top_rating_history)
                .service(info::info::submitted_users)
                .service(info::info::users2)
                .service(info::info::vote_info)
        })
        .bind("localhost:8000")
        .unwrap()
        .run()
        .await;
    } else {
        if let Err(err) = env::var("API_URL") {
            panic!("API_URL is not set");
        }
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(
                "/etc/letsencrypt/live/colledor-api.tk/privkey.pem",
                SslFiletype::PEM,
            )
            .unwrap();
        builder
            .set_certificate_chain_file("/etc/letsencrypt/live/colledor-api.tk/fullchain.pem")
            .unwrap();
        HttpServer::new(|| {
            let cors = Cors::default()
                .allowed_origin_fn(|origin, _req_head| true)
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec!["content-type", "access-control-allow-origin"]);

            App::new()
                .wrap(cors)
                .service(auth::auth::login)
                .service(submit::submit::submit)
                .service(challenge::challenge::vote)
                .service(challenge::challenge::challenge)
                .service(info::info::submission_info)
                .service(info::info::submissions_list)
                .service(info::info::challenge_info)
                .service(info::info::challenges_list)
                .service(info::info::users)
                .service(info::info::top_rating_history)
                .service(info::info::vote_info)
                .service(info::info::submitted_users)
                .service(info::info::users2)
        })
        .bind_openssl(env::var("API_URL").unwrap(), builder)
        .unwrap()
        .run()
        .await;
    }
}
