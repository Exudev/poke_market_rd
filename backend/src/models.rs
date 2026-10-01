use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub email_verified: Option<bool>,
    pub verification_token: Option<String>,
    pub reset_password_token: Option<String>,
    pub reset_password_expires_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub pokemon_name: String,
    pub series: String,
    pub set_name: String,
    pub condition: String,
    pub price_cents: i32,
    pub currency: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostDetailResponse {
    #[serde(flatten)]
    pub post: Post,
    pub seller_username: String,
    pub seller_avatar_url: Option<String>,
    pub seller_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct PostFilter {
    pub sets: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub pokemon_name: String,
    pub series: String,
    pub set_name: String,
    pub condition: String,
    pub price_cents: i32,
    pub currency: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleAuthRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String, // Use raw identifier for 'type' since it's a keyword
    pub message: String,
    pub is_read: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    pub id: Uuid,
    pub post_id: Uuid,
    pub buyer_id: Uuid,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOfferRequest {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOfferStatusRequest {
    pub status: String,
}

pub struct CreateNotificationRequest {
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityPost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub content: String,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub likes_count: Option<i64>,
    pub comments_count: Option<i64>,
    pub user_liked: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommunityPostRequest {
    pub content: String,
    pub image_url: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityComment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}
