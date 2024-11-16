use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;  
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime};
use crate::models::attendance::Attendance;

pub async fn punch_in(attendance: web::Json<Attendance>) -> impl Responder {
    let db = crate::db::connect::get_database().await;
    let collection = db.collection::<Attendance>("attendance");

    let punch_in_time = Utc::now().naive_utc();
    let new_attendance = Attendance {
        punch_in: punch_in_time.into(),
        total_hours: None,
        punch_out: None,
        ..attendance.into_inner()
    };

    match collection.insert_one(new_attendance, None).await {
        Ok(_) => HttpResponse::Ok().body("Punch-in successful"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to punch in"),
    }
}

pub async fn punch_out(attendance_id: web::Path<String>) -> impl Responder {
    let db = crate::db::connect::get_database().await;
    let collection = db.collection::<Attendance>("attendance");

    let punch_out_time = Utc::now(); 
    let system_time = punch_out_time.into(); 
    let bson_punch_out_time = Bson::DateTime(BsonDateTime::from_system_time(system_time)); 
    let filter = doc! { "_id": attendance_id.into_inner() };
    let update = doc! {
        "$set": { "punch_out": bson_punch_out_time },
        "$inc": { "total_hours": 8.0 }
    };

    match collection.update_one(filter, update, None).await {
        Ok(_) => HttpResponse::Ok().body("Punch-out successful"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to punch out"),
    }
}
