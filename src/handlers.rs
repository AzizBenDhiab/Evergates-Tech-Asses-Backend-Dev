use crate::models::Details;
use axum::{extract::Extension, Json};
use std::sync::Arc;
use tokio_postgres::Client;
use validator::Validate;

pub async fn post_details(
    Json(payload): Json<Details>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<&'static str>, (axum::http::StatusCode, String)> {
    if let Err(validation_errors) = payload.validate() {
        let error_message = format!("Validation failed: {:?}", validation_errors);
        return Err((axum::http::StatusCode::BAD_REQUEST, error_message));
    }
    client
        .execute(
            r#"INSERT INTO "details" (name, value) VALUES ($1, $2)"#,
            &[&payload.name, &payload.value],
        )
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json("Success"))
}

pub async fn get_details(
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Vec<Details>>, (axum::http::StatusCode, String)> {
    let rows = client
        .query(r#"SELECT name, value FROM "details""#, &[])
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let details: Vec<Details> = rows
        .into_iter()
        .map(|row| Details {
            name: row.get(0),
            value: row.get(1),
        })
        .collect();

    Ok(Json(details))
}
