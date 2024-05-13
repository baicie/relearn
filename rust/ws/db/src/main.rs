use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: NaiveDateTime,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let course_rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where id = ?"#,
        1i32
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut courses_list = vec![];
    for row in course_rows {
        courses_list.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: row.time,
        })
    }
    println!("{:?}", courses_list);

    Ok(())
}
