mod db;
mod models;
mod controllers;

use actix_web::{web, App, HttpServer};
use controllers::attendace_controller::punch_in;
use controllers::attendace_controller::punch_out;
use controllers::payroll_controller::generate_payroll;
use crate::controllers::employee_controller::*;
use crate::controllers::employee_calender::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/employees", web::post().to(add_employee))
            .route("/employees", web::get().to(get_employees))
            .route("/add_holiday", web::post().to(add_holiday))
            .route("/get_holiday", web::get().to(get_holidays))
            .route("/punch_in", web::post().to(punch_in))
            .route("/punch_out", web::post().to(punch_out))
            .route("/generate_payroll", web::post().to(generate_payroll))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
