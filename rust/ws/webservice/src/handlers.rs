use std::sync::Mutex;

use crate::models::Course;

use super::state::AppState;
use actix_web::{http::StatusCode, web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(response)
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
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

pub async fn add_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Adding course: {:?}", new_course);
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();

    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added successfully.")
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            teacher_id: 1,
            id: None,
            name: "Math".to_string(),
            time: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = add_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
