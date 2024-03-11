use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

//? Book states The error event does not fall within the query(http?) span and we have
//? a better separation of concerns
#[tracing::instrument(
        name="Adding a new subscriber",
        skip(form,pool),
        fields(
            request_id= %Uuid::new_v4(),
            subscriber_email = %form.email,
            subscriber_name = %form.name
        )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(form, pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

//todo spanner
pub async fn insert_subscriber(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions 
                (id,    email,  name, subscribed_at)
        values  ($1,    $2,     $3,   $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute db query -> {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling latter
    })?;
    Ok(())
}
