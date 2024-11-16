use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payroll {
    pub employee_id: String,
    pub month: String,  
    pub year: i32,
    pub total_hours: f64,  
    pub hourly_rate: f64,  
    pub total_salary: f64, 
}
