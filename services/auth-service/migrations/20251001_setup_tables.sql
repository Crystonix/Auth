-- Roles
CREATE TYPE user_role AS ENUM ('user', 'admin');

-- Users
CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   username TEXT NOT NULL,
   avatar TEXT,
   role user_role NOT NULL DEFAULT 'user',
   created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
   updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
   last_login TIMESTAMPTZ,
   login_count INTEGER NOT NULL DEFAULT 0
);

-- Provider accounts
CREATE TYPE oauth_provider AS ENUM ('discord', 'google'); -- extendable
CREATE TABLE user_providers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider oauth_provider NOT NULL,
    provider_user_id TEXT NOT NULL,
    discriminator TEXT,
    avatar TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(provider, provider_user_id)
);
CREATE INDEX idx_user_providers_user_id ON user_providers(user_id);

-- OAuth Tokens
CREATE TABLE oauth_tokens (
  id SERIAL PRIMARY KEY,
  user_provider_id INTEGER NOT NULL REFERENCES user_providers(id) ON DELETE CASCADE,
  encrypted_refresh_token BYTEA NOT NULL,
  refresh_token_nonce BYTEA NOT NULL,
  previous_refresh_token BYTEA,
  previous_refresh_token_nonce BYTEA,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  last_token_rotation TIMESTAMPTZ,
  expires_at TIMESTAMPTZ,
  revoked_at TIMESTAMPTZ,
  CONSTRAINT oauth_tokens_user_provider_id_unique UNIQUE (user_provider_id)
);

CREATE INDEX idx_oauth_tokens_last_rotation ON oauth_tokens(last_token_rotation);
