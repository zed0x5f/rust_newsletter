use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // let mut connection: &mut PgConnection = connection.as_ref();
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id,email,name,subscribed_at)
    values ($1,$2,$3,$4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            println!("Database Error -> {}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
    // println!("{:?}", &form);
}
