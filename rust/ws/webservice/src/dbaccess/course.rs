use crate::error::AppError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::mysql::MySqlPool;

pub async fn get_courses_for_teacher_db(
    pool: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, AppError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course WHERE teacher_id = ?
        "#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, AppError> {
    let row = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course WHERE teacher_id = ? AND id = ?
        "#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(AppError::NotFound("Course not found".to_string()))
    }
}

pub async fn post_new_course_db(
    pool: &MySqlPool,
    new_course: CreateCourse,
) -> Result<Course, AppError> {
    sqlx::query!(
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level
    )
    .execute(pool)
    .await?;

    let row = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course WHERE teacher_id = ? AND name = ?
        "#,
        new_course.teacher_id,
        new_course.name
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
) -> Result<String, AppError> {
    let row = sqlx::query!(
        r#"DELETE FROM course WHERE teacher_id = ? AND id = ?
        "#,
        teacher_id,
        id
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", row))
}

pub async fn update_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, AppError> {
    let current_course_row = get_course_details_db(pool, teacher_id, id).await?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };

    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };

    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };

    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };

    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };

    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };

    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };

    let course_row = sqlx::query_as!(
        Course,
        r#"
        UPDATE course SET name = ?, description = ?, format = ?, structure = ?, duration = ?, price = ?, language = ?, level = ?
        WHERE teacher_id = ? AND id = ?
        "#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id
    ).execute(pool).await;

    let current_course_row = get_course_details_db(pool, teacher_id, id).await?;
    Ok(current_course_row)
}
