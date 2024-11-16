use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc;
use crate::models::calender::Holiday;
use futures_util::stream::StreamExt;

pub async fn add_holiday(holiday: web::Json<Holiday>) -> impl Responder {
    let db = crate::db::connect::get_database().await;
    let collection = db.collection::<Holiday>("holidays");

    match collection.insert_one(holiday.into_inner(), None).await {
        Ok(_) => HttpResponse::Ok().body("Holiday added successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add holiday"),
    }
}

pub async fn get_holidays(year: web::Path<i32>) -> impl Responder {
    let db = crate::db::connect::get_database().await;
    let collection = db.collection::<Holiday>("holidays");
    match collection.find(doc! {"year": year.into_inner()}, None).await {
        Ok(mut cursor) => {
            let mut holidays = Vec::new();
            
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(holiday) => holidays.push(holiday),
                    Err(_) => return HttpResponse::InternalServerError().body("Error processing holiday data"),
                }
            }
            
            HttpResponse::Ok().json(holidays)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch holidays"),
    }
}
