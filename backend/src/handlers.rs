use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;

use crate::auth::create_jwt;
use crate::models::{LoginRequest, RegisterRequest, AuthResponse, User, UserResponse};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let hashed_password = hash(payload.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, verification_token)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, password_hash, google_id, email, bio, avatar_url, phone, created_at, email_verified, verification_token, reset_password_token, reset_password_expires_at
        "#,
        payload.username,
        payload.email,
        hashed_password,
        verification_token
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.is_unique_violation() {
                return (StatusCode::CONFLICT, "Username or Email already exists".to_string());
            }
        }
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    crate::mailer::send_verification_email(&user.email.clone().unwrap_or_default(), &verification_token);

    let token = create_jwt(user.id, user.username.clone());

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Deserialize)]
pub struct VerifyEmailQuery {
    pub token: String,
}

pub async fn verify_email(
    State(pool): State<PgPool>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let result = sqlx::query!(
        r#"
        UPDATE users
        SET email_verified = true, verification_token = NULL
        WHERE verification_token = $1
        "#,
        query.token
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(crate::error::ApiError::BadRequest("Invalid or expired token".to_string()));
    }

    Ok((StatusCode::OK, "Email verified successfully! You can now close this tab."))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, google_id, email, bio, avatar_url, phone, created_at, email_verified, verification_token, reset_password_token, reset_password_expires_at
        FROM users
        WHERE username = $1
        "#,
        payload.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))?;

    let hash = user.password_hash.as_ref().ok_or((StatusCode::UNAUTHORIZED, "Please sign in with Google".to_string()))?;

    let is_valid = verify(payload.password.as_bytes(), hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !is_valid {
        return Err(crate::error::ApiError::Unauthorized("Invalid username or password".to_string()));
    }

    let token = create_jwt(user.id, user.username.clone());

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
        },
    };

    Ok((StatusCode::OK, Json(response)))
}

use crate::auth::Claims;
use axum::Extension;
use crate::models::{CreatePostRequest, Post};

pub async fn create_post(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (user_id, pokemon_name, series, set_name, condition, price_cents, currency, description, image_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, user_id, pokemon_name, series, set_name, condition, price_cents, currency, description, image_url, created_at
        "#,
        claims.sub,
        payload.pokemon_name,
        payload.series,
        payload.set_name,
        payload.condition,
        payload.price_cents,
        payload.currency,
        payload.description,
        payload.image_url
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(post)))
}

use crate::models::PostFilter;
use axum::extract::Query;

pub async fn get_posts(
    State(pool): State<PgPool>,
    Query(filter): Query<PostFilter>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    let sets: Option<Vec<String>> = filter.sets.map(|s| s.split(',').map(|s| s.to_string()).collect());
    let limit = filter.limit.unwrap_or(20);
    let offset = filter.offset.unwrap_or(0);
    let search = filter.search;
    let sort = filter.sort;

    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT id, user_id, pokemon_name, series, set_name, condition, price_cents, currency, description, image_url, created_at
        FROM posts
        WHERE 
            ($1::text[] IS NULL OR set_name = ANY($1::text[]))
            AND ($4::text IS NULL OR pokemon_name ILIKE '%' || $4 || '%')
        ORDER BY 
            CASE WHEN $5 = 'price_asc' THEN price_cents END ASC,
            CASE WHEN $5 = 'price_desc' THEN price_cents END DESC,
            created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        sets.as_deref(),
        limit,
        offset,
        search,
        sort
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(posts)))
}

use crate::models::PostDetailResponse;

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub price: f64,
    pub currency: String,
    pub condition: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

pub async fn update_post(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    let price_cents = (payload.price * 100.0).round() as i32;

    let result = sqlx::query!(
        r#"
        UPDATE posts
        SET price_cents = $1, currency = $2, condition = $3, description = $4, image_url = $5
        WHERE id = $6 AND user_id = $7
        "#,
        price_cents,
        payload.currency,
        payload.condition,
        payload.description,
        payload.image_url,
        id,
        claims.sub
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(crate::error::ApiError::NotFound("Post not found or unauthorized".to_string()));
    }

    Ok(StatusCode::OK)
}

pub async fn delete_post(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2",
        id,
        claims.sub
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(crate::error::ApiError::NotFound("Post not found or unauthorized".to_string()));
    }

    Ok(StatusCode::OK)
}

pub async fn get_post(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, user_id, pokemon_name, series, set_name, condition, price_cents, currency, description, image_url, created_at
        FROM posts
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    let seller = sqlx::query!(
        r#"
        SELECT username, avatar_url, phone
        FROM users
        WHERE id = $1
        "#,
        post.user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = PostDetailResponse {
        post,
        seller_username: seller.username,
        seller_avatar_url: seller.avatar_url,
        seller_phone: seller.phone,
    };

    Ok((StatusCode::OK, Json(response)))
}

use crate::models::GoogleAuthRequest;

use serde::Deserialize;

#[derive(Deserialize)]
struct GoogleTokenInfo {
    sub: String,
    email: String,
    name: String,
}

pub async fn google_auth(
    State(pool): State<PgPool>,
    Json(payload): Json<GoogleAuthRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://oauth2.googleapis.com/tokeninfoeid_token={}", payload.token))
        .send()
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, "Invalid Google token".to_string()))?;

    if !res.status().is_success() {
        return Err(crate::error::ApiError::Unauthorized("Invalid Google token".to_string()));
    }

    let token_info: GoogleTokenInfo = res.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse Google response".to_string()))?;

    // Check if user exists by google_id
    let existing_user = sqlx::query_as!(
        User,
        r#"SELECT id, username, password_hash, google_id, email, bio, avatar_url, phone, created_at, email_verified, verification_token, reset_password_token, reset_password_expires_at FROM users WHERE google_id = $1"#,
        token_info.sub
    ).fetch_optional(&pool).await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = if let Some(u) = existing_user {
        u
    } else {
        // Create new user
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, google_id, email)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, google_id, email, bio, avatar_url, phone, created_at, email_verified, verification_token, reset_password_token, reset_password_expires_at
            "#,
            token_info.name,
            token_info.sub,
            token_info.email
        ).fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    };

    let token = create_jwt(user.id, user.username.clone());

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
        },
    };

    Ok((StatusCode::OK, Json(response)))
}

use crate::models::UpdateProfileRequest;

pub async fn get_me(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, google_id, email, bio, avatar_url, phone, created_at, email_verified, verification_token, reset_password_token, reset_password_expires_at
        FROM users
        WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_profile(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET username = COALESCE($1, username),
            bio = COALESCE($2, bio),
            avatar_url = COALESCE($3, avatar_url),
            phone = COALESCE($4, phone)
        WHERE id = $5
        RETURNING id, username, password_hash, google_id, email, bio, avatar_url, phone, created_at, email_verified, verification_token, reset_password_token, reset_password_expires_at
        "#,
        payload.username,
        payload.bio,
        payload.avatar_url,
        payload.phone,
        claims.sub
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(user)))
}

#[derive(Deserialize)]
pub struct PaginationFilter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn get_my_posts(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Query(filter): Query<PaginationFilter>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let limit = filter.limit.unwrap_or(20);
    let offset = filter.offset.unwrap_or(0);

    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT id, user_id, pokemon_name, series, set_name, condition, price_cents, currency, description, image_url, created_at
        FROM posts
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        claims.sub,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(posts)))
}

use axum::extract::Multipart;

pub async fn upload_image(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    // Check if Cloudinary is configured
    let cloudinary_url = std::env::var("CLOUDINARY_URL").ok();
    
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        if let Some(mut file_name) = field.file_name().map(|s| s.to_string()) {
            if file_name.is_empty() {
                file_name = "image.png".to_string();
            }
            
            let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            
            if let Some(c_url) = cloudinary_url.clone() {
                // Cloudinary URL format: cloudinary://api_key:api_secret@cloud_name
                let without_scheme = c_url.trim_start_matches("cloudinary://");
                let parts: Vec<&str> = without_scheme.split('@').collect();
                if parts.len() == 2 {
                    let creds: Vec<&str> = parts[0].split(':').collect();
                    if creds.len() == 2 {
                        let api_key = creds[0];
                        let api_secret = creds[1];
                        let cloud_name = parts[1];
                        
                        let timestamp = chrono::Utc::now().timestamp();
                        
                        use sha1::{Sha1, Digest};
                        let to_sign = format!("moderation=aws_rek&timestamp={}{}", timestamp, api_secret);
                        let mut hasher = Sha1::new();
                        hasher.update(to_sign.as_bytes());
                        let signature = hex::encode(hasher.finalize());
                        
                        let form = reqwest::multipart::Form::new()
                            .part("file", reqwest::multipart::Part::bytes(data.to_vec()).file_name(file_name))
                            .text("api_key", api_key.to_string())
                            .text("timestamp", timestamp.to_string())
                            .text("moderation", "aws_rek")
                            .text("signature", signature);
                            
                        let url = format!("https://api.cloudinary.com/v1_1/{}/image/upload", cloud_name);
                        let client = reqwest::Client::new();
                        let res = client.post(&url)
                            .multipart(form)
                            .send()
                            .await
                            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                            
                        let body: serde_json::Value = res.json()
                            .await
                            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                            
                        if let Some(err) = body.get("error") {
                            return Err(crate::error::ApiError::BadRequest(err.get("message").and_then(|m| m.as_str()).unwrap_or("Cloudinary error").to_string()));
                        }
                            
                        if let Some(secure_url) = body.get("secure_url").and_then(|v| v.as_str()) {
                            return Ok((StatusCode::OK, Json(serde_json::json!({ "url": secure_url }))));
                        }
                    }
                }
                return Err(crate::error::ApiError::Internal("Invalid CLOUDINARY_URL format".to_string()));
            } else {
                // Fallback to local uploads
                let ext = std::path::Path::new(&file_name)
                    .extension()
                    .and_then(|v| v.to_str())
                    .unwrap_or("png");
                
                let unique_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
                let path = std::path::Path::new("uploads").join(&unique_name);
                
                tokio::fs::write(&path, data).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                
                let url = format!("http://localhost:8080/uploads/{}", unique_name);
                return Ok((StatusCode::OK, Json(serde_json::json!({ "url": url }))));
            }
        }
    }
    Err(crate::error::ApiError::BadRequest("No file found".to_string()))
}

use crate::models::Notification;

pub async fn get_notifications(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let notifications = sqlx::query_as!(
        Notification,
        r#"
        SELECT id, user_id, type as "type!", message, is_read, created_at
        FROM notifications
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        claims.sub
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(notifications)))
}

pub async fn mark_notifications_read(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    sqlx::query!(
        r#"
        UPDATE notifications
        SET is_read = true
        WHERE user_id = $1
        "#,
        claims.sub
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

pub async fn track_whatsapp_click(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(post_id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let post = sqlx::query!(
        "SELECT user_id, pokemon_name FROM posts WHERE id = $1",
        post_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    // Don't notify if the user clicked their own WhatsApp button somehow
    if post.user_id != claims.sub {
        let message = format!("Someone is interested in your {}! They just clicked the WhatsApp button.", post.pokemon_name);
        
        sqlx::query!(
            r#"
            INSERT INTO notifications (user_id, type, message)
            VALUES ($1, 'whatsapp_click', $2)
            "#,
            post.user_id,
            message
        )
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

pub async fn toggle_favorite(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(post_id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let exists = sqlx::query!(
        "SELECT 1 as exists FROM favorites WHERE user_id = $1 AND post_id = $2",
        claims.sub,
        post_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if exists.is_some() {
        sqlx::query!(
            "DELETE FROM favorites WHERE user_id = $1 AND post_id = $2",
            claims.sub,
            post_id
        )
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok((StatusCode::OK, Json(serde_json::json!({ "favorited": false }))))
    } else {
        sqlx::query!(
            "INSERT INTO favorites (user_id, post_id) VALUES ($1, $2)",
            claims.sub,
            post_id
        )
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok((StatusCode::OK, Json(serde_json::json!({ "favorited": true }))))
    }
}

pub async fn get_my_favorites(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Query(filter): Query<PaginationFilter>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let limit = filter.limit.unwrap_or(20);
    let offset = filter.offset.unwrap_or(0);

    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT p.id, p.user_id, p.pokemon_name, p.series, p.set_name, p.condition, p.price_cents, p.currency, p.description, p.image_url, p.created_at
        FROM posts p
        JOIN favorites f ON p.id = f.post_id
        WHERE f.user_id = $1
        ORDER BY f.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        claims.sub,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(posts)))
}

use crate::models::{Offer, CreateOfferRequest, UpdateOfferStatusRequest};

pub async fn create_offer(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<CreateOfferRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    let amount_cents = (payload.amount * 100.0).round() as i32;

    // Verify post exists and isn't owned by the buyer
    let post = sqlx::query!("SELECT user_id FROM posts WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let post = match post {
        Some(p) => p,
        None => return Err(crate::error::ApiError::NotFound("Post not found".to_string())),
    };

    if post.user_id == claims.sub {
        return Err(crate::error::ApiError::BadRequest("Cannot make an offer on your own post".to_string()));
    }

    let offer = sqlx::query_as!(
        Offer,
        r#"
        INSERT INTO offers (post_id, buyer_id, amount_cents, currency)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        id,
        claims.sub,
        amount_cents,
        payload.currency
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create a notification for the seller
    let seller_id = post.user_id;
    let _ = sqlx::query!(
        "INSERT INTO notifications (user_id, type, message) VALUES ($1, $2, $3)",
        seller_id,
        "offer_received",
        format!("You received a new offer of {} {:.2} from {}", offer.currency, offer.amount_cents as f64 / 100.0, claims.username)
    )
    .execute(&pool)
    .await;

    Ok((StatusCode::CREATED, Json(offer)))
}

use serde::Serialize;

#[derive(Serialize)]
pub struct OfferWithDetails {
    pub id: uuid::Uuid,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub post_id: uuid::Uuid,
    pub pokemon_name: String,
    pub image_url: Option<String>,
    pub buyer_username: String,
    pub type_str: Option<String>, // "sent" or "received"
}

pub async fn get_my_offers(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    // Fetch offers the user has sent OR received
    let offers = sqlx::query_as!(
        OfferWithDetails,
        r#"
        SELECT 
            o.id, o.amount_cents, o.currency, o.status, o.created_at, o.post_id,
            p.pokemon_name, p.image_url,
            u.username as buyer_username,
            CASE WHEN o.buyer_id = $1 THEN 'sent' ELSE 'received' END as type_str
        FROM offers o
        JOIN posts p ON o.post_id = p.id
        JOIN users u ON o.buyer_id = u.id
        WHERE o.buyer_id = $1 OR p.user_id = $1
        ORDER BY o.created_at DESC
        "#,
        claims.sub
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(offers)))
}

pub async fn update_offer_status(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<UpdateOfferStatusRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    if payload.status != "accepted" && payload.status != "rejected" {
        return Err(crate::error::ApiError::BadRequest("Invalid status".to_string()));
    }

    // Must be the seller to accept/reject
    let result = sqlx::query!(
        r#"
        UPDATE offers o
        SET status = $1
        FROM posts p
        WHERE o.id = $2 AND o.post_id = p.id AND p.user_id = $3
        RETURNING o.buyer_id, p.pokemon_name
        "#,
        payload.status,
        id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(record) = result {
        // Notify buyer
        let _ = sqlx::query!(
            "INSERT INTO notifications (user_id, type, message) VALUES ($1, $2, $3)",
            record.buyer_id,
            "offer_updated",
            format!("Your offer for {} was {}", record.pokemon_name, payload.status)
        )
        .execute(&pool)
        .await;
        Ok(StatusCode::OK)
    } else {
        Err(crate::error::ApiError::NotFound("Offer not found or unauthorized".to_string()))
    }
}


#[derive(Deserialize)]
pub struct MarketPriceQuery {
    pub name: String,
    pub set: Option<String>,
}

#[derive(Serialize)]
pub struct MarketPriceResponse {
    pub average_sell_price: Option<f64>,
    pub low_price: Option<f64>,
    pub high_price: Option<f64>,
}

pub async fn get_market_price(
    Query(query): Query<MarketPriceQuery>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    // We will query api.pokemontcg.io
    // Format: https://api.pokemontcg.io/v2/cardseq=name:"charizard" set.name:"base"
    
    let mut q = format!("name:\"*{:?}*\"", query.name.replace('"', ""));
    if let Some(set_name) = &query.set {
        q = format!("{} set.name:\"*{:?}*\"", q, set_name.replace('"', ""));
    }

    let q_encoded = q.replace(" ", "%20");
    let url = format!("https://api.pokemontcg.io/v2/cardseq={}", q_encoded);

    let client = reqwest::Client::new();
    let res = client.get(&url)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let body: serde_json::Value = res.json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Try to extract tcgplayer prices from the first matching card
    let cards = body.get("data").and_then(|d| d.as_array());
    
    if let Some(cards) = cards {
        if let Some(first_card) = cards.first() {
            if let Some(tcgplayer) = first_card.get("tcgplayer") {
                if let Some(prices) = tcgplayer.get("prices") {
                    // Try holofoil or normal
                    let price_data = prices.get("holofoil").or_else(|| prices.get("normal"));
                    
                    if let Some(p) = price_data {
                        let avg = p.get("market").and_then(|v| v.as_f64());
                        let low = p.get("low").and_then(|v| v.as_f64());
                        let high = p.get("high").and_then(|v| v.as_f64());
                        
                        return Ok((StatusCode::OK, Json(MarketPriceResponse {
                            average_sell_price: avg,
                            low_price: low,
                            high_price: high,
                        })));
                    }
                }
            }
        }
    }

    // Return empty if not found
    Ok((StatusCode::OK, Json(MarketPriceResponse {
        average_sell_price: None,
        low_price: None,
        high_price: None,
    })))
}

use crate::models::{CommunityPost, CreateCommunityPostRequest, CommunityComment, CreateCommentRequest};

#[derive(Deserialize)]
pub struct CommunityPostFilter {
    pub category: Option<String>,
}

pub async fn get_community_posts(
    State(pool): State<PgPool>,
    // Optional auth token to see if user liked
    req: axum::extract::Request,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    // Check if there is an auth token in the request header
    let mut current_user_id: Option<uuid::Uuid> = None;
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                // In a real app we would properly decode and verify, but since we use extensions normally,
                // we can just decode the payload without verification here since it's just for UI state
                // Or better yet, we can try to parse the JWT payload.
                let parts: Vec<&str> = token.split('.').collect();
                if parts.len() == 3 {
                    if let Ok(payload_bytes) = base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[1]) {
                        if let Ok(claims) = serde_json::from_slice::<serde_json::Value>(&payload_bytes) {
                            if let Some(sub) = claims.get("sub").and_then(|v| v.as_str()) {
                                if let Ok(uid) = uuid::Uuid::parse_str(sub) {
                                    current_user_id = Some(uid);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Parse query params manually since we consumed the request
    let query_str = req.uri().query().unwrap_or("");
    let mut category: Option<String> = None;
    for param in query_str.split('&') {
        let kv: Vec<&str> = param.split('=').collect();
        if kv.len() == 2 && kv[0] == "category" {
            category = Some(kv[1].to_string());
        }
    }

    let category = category.filter(|c| c != "All" && c != "undefined");

    let posts = sqlx::query_as!(
        CommunityPost,
        r#"
        SELECT 
            c.id, c.user_id, u.username, u.avatar_url, c.content, c.image_url, c.category, c.created_at,
            (SELECT COUNT(*) FROM community_likes cl WHERE cl.post_id = c.id) as likes_count,
            (SELECT COUNT(*) FROM community_comments cc WHERE cc.post_id = c.id) as comments_count,
            EXISTS(SELECT 1 FROM community_likes cl WHERE cl.post_id = c.id AND cl.user_id = $1) as user_liked
        FROM community_posts c
        JOIN users u ON c.user_id = u.id
        WHERE ($2::text IS NULL OR c.category = $2)
        ORDER BY c.created_at DESC
        LIMIT 50
        "#,
        current_user_id,
        category
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn create_community_post(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateCommunityPostRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let post = sqlx::query_as!(
        CommunityPost,
        r#"
        WITH inserted AS (
            INSERT INTO community_posts (user_id, content, image_url, category)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, content, image_url, category, created_at
        )
        SELECT 
            i.id, i.user_id, u.username, u.avatar_url, i.content, i.image_url, i.category, i.created_at,
            0::bigint as likes_count, 0::bigint as comments_count, false as user_liked
        FROM inserted i
        JOIN users u ON i.user_id = u.id
        "#,
        claims.sub,
        payload.content,
        payload.image_url,
        payload.category.unwrap_or_else(|| "General".to_string())
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn toggle_community_like(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    
    // Check if like exists
    let existing = sqlx::query!("SELECT user_id FROM community_likes WHERE user_id = $1 AND post_id = $2", claims.sub, id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing.is_some() {
        // Unlike
        sqlx::query!("DELETE FROM community_likes WHERE user_id = $1 AND post_id = $2", claims.sub, id)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    } else {
        // Like
        sqlx::query!("INSERT INTO community_likes (user_id, post_id) VALUES ($1, $2)", claims.sub, id)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

pub async fn get_community_comments(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let comments = sqlx::query_as!(
        CommunityComment,
        r#"
        SELECT c.id, c.post_id, c.user_id, u.username, u.avatar_url, c.content, c.created_at
        FROM community_comments c
        JOIN users u ON c.user_id = u.id
        WHERE c.post_id = $1
        ORDER BY c.created_at ASC
        "#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(comments)))
}

pub async fn create_community_comment(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let comment = sqlx::query_as!(
        CommunityComment,
        r#"
        WITH inserted AS (
            INSERT INTO community_comments (post_id, user_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, post_id, user_id, content, created_at
        )
        SELECT i.id, i.post_id, i.user_id, u.username, u.avatar_url, i.content, i.created_at
        FROM inserted i
        JOIN users u ON i.user_id = u.id
        "#,
        id,
        claims.sub,
        payload.content
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn forgot_password(
    State(pool): State<PgPool>,
    Json(payload): Json<crate::models::ForgotPasswordRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let user = sqlx::query!(
        "SELECT id, email FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await?;

    if let Some(user) = user {
        let reset_token = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(1);

        sqlx::query!(
            "UPDATE users SET reset_password_token = $1, reset_password_expires_at = $2 WHERE id = $3",
            reset_token,
            expires_at,
            user.id
        )
        .execute(&pool)
        .await?;

        if let Some(email) = user.email {
            crate::mailer::send_password_reset_email(&email, &reset_token);
        }
    }

    // Always return OK to prevent email enumeration
    Ok((StatusCode::OK, "If that email exists, a reset link has been sent."))
}

pub async fn reset_password(
    State(pool): State<PgPool>,
    Json(payload): Json<crate::models::ResetPasswordRequest>,
) -> Result<impl IntoResponse, crate::error::ApiError> {
    let user = sqlx::query!(
        "SELECT id, reset_password_expires_at FROM users WHERE reset_password_token = $1",
        payload.token
    )
    .fetch_optional(&pool)
    .await?;

    if let Some(user) = user {
        if let Some(expires_at) = user.reset_password_expires_at {
            if expires_at < chrono::Utc::now().naive_utc() {
                return Err(crate::error::ApiError::BadRequest("Reset token has expired".to_string()));
            }

            let hashed_password = bcrypt::hash(payload.new_password.as_bytes(), bcrypt::DEFAULT_COST)
                .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;

            sqlx::query!(
                "UPDATE users SET password_hash = $1, reset_password_token = NULL, reset_password_expires_at = NULL WHERE id = $2",
                hashed_password,
                user.id
            )
            .execute(&pool)
            .await?;

            return Ok((StatusCode::OK, "Password has been successfully reset."));
        }
    }

    Err(crate::error::ApiError::BadRequest("Invalid or expired reset token".to_string()))
}
