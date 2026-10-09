ALTER TABLE users ADD COLUMN email_verified BOOLEAN DEFAULT false;
ALTER TABLE users ADD COLUMN verification_token VARCHAR(255);
ALTER TABLE users ADD COLUMN reset_password_token VARCHAR(255);
ALTER TABLE users ADD COLUMN reset_password_expires_at TIMESTAMP;
