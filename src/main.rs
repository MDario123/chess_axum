use axum::{
    routing::{get, post},
    Router,
};
use core::time::Duration;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
#[cfg(debug_assertions)]
use tracing::Level;

pub(crate) mod game;
pub(crate) mod password_checker;
mod route;

#[tokio::main]
#[tracing::instrument]
async fn main() {
    // initialize tracing
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .pretty()
        .init();
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt().compact().pretty().init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@127.0.0.1/chess?sslmode=disable".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with a route
    let app = Router::new()
        .route("/user/login", get(route::user::get::handler))
        .route("/user/register", post(route::user::post::handler))
        .route("/invite", post(route::game::invite::handler))
        .route("/invited", get(route::game::invited::handler))
        .with_state(pool);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
