use super::models::*;
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;

pub async fn get_courses_for_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"
        SELECT * FROM course WHERE teacher_id = ?
        "#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()
        .map(|r| Course {
            id: Some(r.id as usize),
            teacher_id: r.teacher_id as usize,
            name: r.name.clone(),
            time: Some(r.time),
        })
        .collect()
}

pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Course {
    let row = sqlx::query!(
        r#"
        SELECT * FROM course WHERE teacher_id = ? AND id = ?
        "#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        id: Some(row.id as usize),
        teacher_id: row.teacher_id as usize,
        name: row.name.clone(),
        time: Some(row.time),
    }
}
