use std::sync::Mutex;

use crate::models::Course;

use super::state::AppState;
use actix_web::{http::StatusCode, web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(response)
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize)>,
) -> HttpResponse {
    let teacher_id: usize = params.0;
    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for this teacher.".to_string())
    }
}

#[actix_rt::test]
async fn get_all_courses_success() {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let teacher_id: web::Path<usize> = web::Path::from(1);
    let resp = get_courses_for_teacher(app_state, teacher_id).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
