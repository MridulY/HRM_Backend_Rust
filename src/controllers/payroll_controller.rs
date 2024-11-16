use actix_web::{web, HttpResponse, Responder};
use crate::models::payroll::Payroll;

pub async fn generate_payroll(employee_id: web::Path<String>) -> impl Responder {
    let db = crate::db::connect::get_database().await;
    let collection = db.collection::<Payroll>("payroll");

    // Calculate total hours worked in a month (simplified example)
    let total_hours = 160.0;  // Hardcoded, you can replace with real calculations based on attendance data

    let hourly_rate = 20.0;  // Replace with employee's actual hourly rate
    let total_salary = total_hours * hourly_rate;

    let payroll = Payroll {
        employee_id: employee_id.into_inner(),
        month: "November".to_string(),
        year: 2024,
        total_hours,
        hourly_rate,
        total_salary,
    };

    match collection.insert_one(payroll, None).await {
        Ok(_) => HttpResponse::Ok().body("Payroll generated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to generate payroll"),
    }
}
