use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::chrono, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

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
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    return match sqlx::query!(
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
    .instrument(query_span)
    .await
    {
        //match arms
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            // println!("Database Error -> {:?}", error);
            //? this falls outside the query span book says we ill fix this
            tracing::error!("Failed to execute db query -> {:?}", error);
            HttpResponse::InternalServerError().finish()
        }
    };
}
