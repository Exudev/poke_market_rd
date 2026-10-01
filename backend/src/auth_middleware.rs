use axum::{
    body::Body, extract::Request, http::{header, StatusCode}, middleware::Next, response::IntoResponse
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::auth::Claims;

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    let token = if let Some(t) = auth_header {
        t
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key_change_me_in_production".to_string());
    
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
