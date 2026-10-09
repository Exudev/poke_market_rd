use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn get_base_url() -> String {
    env::var("BASE_URL")
        .or_else(|_| env::var("FRONTEND_URL"))
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .trim_end_matches('/')
        .to_string()
}

pub fn send_verification_email(to_email: &str, token: &str) {
    let base_url = get_base_url();
    let from_sender = env::var("SMTP_FROM").unwrap_or_else(|_| "PokéMart <onboarding@resend.dev>".to_string());

    let email = Message::builder()
        .from(from_sender.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Welcome to PokéMart! Please verify your email")
        .body(format!(
            "Welcome to PokéMart!\n\nPlease click the link below to verify your email address:\n{}/verify-email?token={}",
            base_url, token
        ))
        .unwrap();

    let smtp_user = env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = env::var("SMTP_PASS").unwrap_or_default();
    let smtp_host = env::var("SMTP_HOST").unwrap_or_default();

    if smtp_user.is_empty() || smtp_host.is_empty() {
        println!("------------------------------------------------");
        println!("EMAIL STUB: Sending verification email to {}", to_email);
        println!("Subject: Welcome to PokéMart! Please verify your email");
        println!("Token: {}", token);
        println!("Link: {}/verify-email?token={}", base_url, token);
        println!("------------------------------------------------");
        return;
    }

    let creds = lettre::transport::smtp::authentication::Credentials::new(smtp_user, smtp_pass);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}

pub fn send_password_reset_email(to_email: &str, token: &str) {
    let base_url = get_base_url();
    let from_sender = env::var("SMTP_FROM").unwrap_or_else(|_| "PokéMart <onboarding@resend.dev>".to_string());

    let email = Message::builder()
        .from(from_sender.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Reset your PokéMart Password")
        .body(format!(
            "You requested a password reset.\n\nPlease click the link below to set a new password:\n{}/reset-password?token={}",
            base_url, token
        ))
        .unwrap();

    let smtp_user = env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = env::var("SMTP_PASS").unwrap_or_default();
    let smtp_host = env::var("SMTP_HOST").unwrap_or_default();

    if smtp_user.is_empty() || smtp_host.is_empty() {
        println!("------------------------------------------------");
        println!("EMAIL STUB: Sending password reset email to {}", to_email);
        println!("Subject: Reset your PokéMart Password");
        println!("Token: {}", token);
        println!("Link: {}/reset-password?token={}", base_url, token);
        println!("------------------------------------------------");
        return;
    }

    let creds = lettre::transport::smtp::authentication::Credentials::new(smtp_user, smtp_pass);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Password reset email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_trailing_slash() {
        assert_eq!("https://pokemarketrd.com/".trim_end_matches('/'), "https://pokemarketrd.com");
        assert_eq!("https://pokemarketrd.com".trim_end_matches('/'), "https://pokemarketrd.com");
    }

    #[test]
    fn test_get_base_url() {
        let base = get_base_url();
        assert!(!base.is_empty());
        assert!(!base.ends_with('/'));
    }
}

