use actix_web::{web, Responder, HttpResponse};
use db::connect::*;
use models::employee::*;
use mongodb::bson::doc;
use futures_util::stream::StreamExt;

use crate::{db, models}; 

pub async fn add_employee(employee: web::Json<Employee>) -> impl Responder {
    let db = get_database().await;
    let collection = db.collection::<Employee>("employees");

    match collection.insert_one(employee.into_inner(), None).await {
        Ok(_) => HttpResponse::Ok().body("Employee added successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add employee"),
    }
}

pub async fn get_employees() -> impl Responder {
    let db = get_database().await;
    let collection = db.collection::<Employee>("employees");

    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut employees = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(employee) => employees.push(employee),
                    Err(_) => return HttpResponse::InternalServerError().body("Error while processing documents"),
                }
            }
            HttpResponse::Ok().json(employees)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch employees"),
    }
}
