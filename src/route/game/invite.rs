use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, PgPool};
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Json(payload): Json<Invitation>,
) -> StatusCode {
    info!("Inviting");

    let res = sqlx::query!(
        "
        INSERT INTO games.v_pending_invites (inviter, invited)
        VALUES ($1, $2)
        ",
        payload.inviter,
        payload.invited,
    )
    .execute(&postgres)
    .await;

    match res {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            if err
                .as_database_error()
                .is_some_and(|err| err.kind() == ErrorKind::UniqueViolation)
            {
                StatusCode::NOT_ACCEPTABLE
            } else {
                error!("Error inviting: {err}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Invitation {
    inviter: String,
    invited: String,
}
