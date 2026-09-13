-- Add migration script here

ALTER TABLE users
ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE email_verification_tokens (
  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  token_hash BYTEA NOT NULL UNIQUE,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


