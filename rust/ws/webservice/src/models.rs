use actix_web::web::{self, Form};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time:Option<NaiveDateTime>
}

impl Form<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
         Course{
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time
         }
    }
}