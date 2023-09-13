use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, PgPool};
use tracing::{error, info};

#[tracing::instrument]
pub(crate) async fn handler(
    // database connection pool
    State(pool): State<PgPool>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> StatusCode {
    info!("Starting!");
    // Store information in the database
    let query_res = sqlx::query!(
        "
        INSERT INTO 
        users.basic_info(username, password) 
        VALUES($1, $2)
        ",
        &payload.username,
        payload.password,
    )
    .execute(&pool)
    .await;

    match query_res {
        Ok(_) => {
            info!("User {} created succesfully", payload.username);
            StatusCode::CREATED
        }
        Err(err) => {
            if err
                .as_database_error()
                .is_some_and(|err| err.kind() == ErrorKind::UniqueViolation)
            {
                info!("User {} already exists", payload.username);
                StatusCode::NOT_ACCEPTABLE
            } else {
                error!("Error inserting to database: {err}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
pub(crate) struct CreateUser {
    username: String,
    password: String,
}
