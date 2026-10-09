use axum::{routing::{get, post, put, delete}, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;

mod auth;
mod db;
mod handlers;
mod models;
mod mailer;
mod error;

mod auth_middleware;

use tower_http::cors::{Any, CorsLayer};

pub fn build_app(pool: sqlx::PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let protected_routes = Router::new()
        .route("/api/posts", post(handlers::create_post))
        .route("/api/posts/:id/edit", put(handlers::update_post))
        .route("/api/posts/:id/delete", delete(handlers::delete_post))
        .route("/api/users/me", get(handlers::get_me).put(handlers::update_profile))
        .route("/api/users/me/posts", get(handlers::get_my_posts))
        .route("/api/users/me/favorites", get(handlers::get_my_favorites))
        .route("/api/posts/:id/favorite", post(handlers::toggle_favorite))
        .route("/api/posts/:id/offers", post(handlers::create_offer))
        .route("/api/users/me/offers", get(handlers::get_my_offers))
        .route("/api/offers/:id", put(handlers::update_offer_status))
        .route("/api/community", post(handlers::create_community_post))
        .route("/api/community/:id/like", post(handlers::toggle_community_like))
        .route("/api/community/:id/comments", post(handlers::create_community_comment))
        .route("/api/upload", post(handlers::upload_image))
        .route("/api/notifications", get(handlers::get_notifications).put(handlers::mark_notifications_read))
        .route("/api/posts/:id/whatsapp_click", post(handlers::track_whatsapp_click))
        .route_layer(axum::middleware::from_fn(auth_middleware::auth_middleware))
        .with_state(pool.clone());

    Router::new()
        .nest_service("/uploads", tower_http::services::ServeDir::new("uploads"))
        .route("/", get(handlers::health_check))
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/verify-email", get(handlers::verify_email))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/google", post(handlers::google_auth))
        .route("/api/auth/forgot-password", post(handlers::forgot_password))
        .route("/api/auth/reset-password", post(handlers::reset_password))

        .route("/api/posts", get(handlers::get_posts))
        .route("/api/posts/:id", get(handlers::get_post))
        .route("/api/market-price", get(handlers::get_market_price))
        .route("/api/community", get(handlers::get_community_posts))
        .route("/api/community/:id/comments", get(handlers::get_community_comments))
        .with_state(pool)
        .merge(protected_routes)
        .layer(cors)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/poke_market".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run database migrations automatically on startup
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let app = build_app(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt; // for `oneshot`
    use http_body_util::BodyExt; // for `collect`

    #[tokio::test]
    async fn test_health_check() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/poke_market".to_string());
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to db");

        let app = build_app(pool);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"OK");
    }

    #[tokio::test]
    async fn test_market_price_returns_ok() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/poke_market".to_string());
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to db");

        let app = build_app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/market-price?name=charizard&set=base")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
