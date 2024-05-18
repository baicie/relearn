use std::sync::Mutex;

use crate::{db_access::*, models::Course};

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
    let teacher_id = i32::try_from(params.into_inner()).unwrap();
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn add_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

pub async fn get_course_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    let course = get_course_details_db(&app_state.db, teacher_id, course_id).await;
    HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let dd_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: dd_pool,
        });

        let teacher_id: web::Path<usize> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let dd_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: dd_pool,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            id: Some(3),
            name: "Math".to_string(),
            time: None,
        });
        let resp = add_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_by_id_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let dd_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: dd_pool,
        });

        let params = web::Path::from((1, 1));
        let resp = get_course_by_id(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
