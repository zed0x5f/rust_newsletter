use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscribeEmail, SubscriberName};

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
            // request_id= %Uuid::new_v4(),
            subscriber_email = %form.email,
            subscriber_name = %form.name
        )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let email = match SubscribeEmail::parse(form.0.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let new_subscriber = NewSubscriber { email, name };

    match insert_subscriber(pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    // form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    let name: &str = new_subscriber.name.as_ref();
    let email: &str = new_subscriber.email.as_ref();

    sqlx::query!(
        r#"
        INSERT INTO subscriptions 
                (id,    email,  name, subscribed_at)
        values  ($1,    $2,     $3,   $4)
        "#,
        Uuid::new_v4(),
        email,
        name,
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
