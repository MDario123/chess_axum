use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use rand::distributions::{Alphanumeric, DistString};
use sqlx::PgPool;
use tracing::error;

#[cfg(test)]
mod test;

pub(crate) fn is_pass_equivalent(a: &str, b: &str) -> bool {
    a == b
}

pub(crate) fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}

#[tracing::instrument]
pub(crate) async fn auth<B: std::fmt::Debug>(
    State(postgres): State<PgPool>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let s = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let s = s.to_str().map_err(|_| StatusCode::BAD_REQUEST)?.to_string();

    let res = sqlx::query_as!(
        LoggedUser,
        "
        SELECT id, username
        FROM users.token t
            JOIN users.basic_info u ON u.id = t.user_id
        WHERE token = $1
          AND expiration > now()
        ",
        s,
    )
    .fetch_optional(&postgres)
    .await
    .map_err(|err| {
        error!("Error checking token {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(res);

    Ok(next.run(req).await)
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct LoggedUser {
    id: i64,
    username: String,
}

impl LoggedUser {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn username(&self) -> &String {
        &self.username
    }
}
