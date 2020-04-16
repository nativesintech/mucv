extern crate chrono;

use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct FormData {}

#[derive(Serialize)]
struct Response {
    response_type: String,
    text: String,
}

fn number_to_day_of_month(day: u32) -> String {
    let days = [
        "'svhvmkat",
        "'svhokkolat",
        "'svtuccēnat",
        "'sostat",
        "'svcahkepat",
        "'svpakat",
        "eskolvpakat",
        "escenvpokakat",
        "'sostvkakat",
        "'svpalat",
        "palen 'svhvmkat",
        "palen 'svhokkolat",
        "palen 'svtuccēnat",
        "palen 'sostat",
        "palen 'svcahkepat",
        "palen 'svpakat",
        "palen eskolvpakat",
        "palen escenvpokakat",
        "palen 'sostvkakat",
        "pale 'svhokkolat",
        "pale-hokkolen 'svhvmkat",
        "pale-hokkolen 'svhokkolat",
        "pale-hokkolen 'svtuccēnat",
        "pale-hokkolen 'svostat",
        "pale-hokkolen 'svcahkepat",
        "pale-hokkolen 'svpakat",
        "pale-hokkolen eskolvpakat",
        "pale-hokkolen escenvpakat",
        "pale-hokkolen 'sostvkakat",
        "pale 'svtuccēnat",
        "pale-tuccēnan 'svhvmkat",
    ];

    let result = *days.get(day as usize).unwrap();

    result.to_string()
}

fn number_to_month(month: u32) -> String {
    let months = [
        "rvfocuse",
        "hotvle hvse",
        "tasacuce",
        "tasace rakko",
        "ke hvse",
        "kvco hvse",
        "hiyuce",
        "hiyo rakko",
        "otowoskuce",
        "otowosko rakko",
        "ehole",
        "rvfo rakko",
    ];

    let result = *months.get(month as usize).unwrap();

    result.to_string()
}

fn number_to_day_of_week(day: u32) -> String {
    let days = [
        "tacakuce",
        "tvcakuce enhayvtke",
        "mvnte enhayvtke",
        "ennvrkvpv",
        "ennvrkvpv enhayvtke",
        "okkoskv nettv",
        "tacakcuse",
    ];

    let result = *days.get(day as usize).unwrap();

    result.to_string()
}

#[post("/")]
async fn mucv(_form: web::Form<FormData>) -> impl Responder {
    dotenv().ok();

    let date = chrono::Utc::now();

    let month = date.month0();
    let day_of_month = date.day0();
    let day_of_week = date.weekday().num_days_from_sunday();

    let date = format!(
        "mucv nettv {} {} {} os",
        number_to_day_of_week(day_of_week),
        number_to_month(month),
        number_to_day_of_month(day_of_month)
    );

    let response = Response {
        response_type: "in_channel".to_string(),
        text: date,
    };

    HttpResponse::Ok()
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("SLACK_TOKEN").expect("No slack token")
            ),
        )
        .header(header::CONTENT_TYPE, "application/json")
        .json(response)
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("APP_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or(8080.to_string());

    let host_and_port = format!("{}:{}", host, port);

    HttpServer::new(|| App::new().service(mucv).service(test))
        .bind(host_and_port)?
        .run()
        .await
}
