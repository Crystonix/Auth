CREATE TYPE user_role AS ENUM ('user', 'admin');

CREATE TABLE users (
   id TEXT PRIMARY KEY,          -- Discord ID
   username TEXT NOT NULL,
   discriminator TEXT NOT NULL,
   avatar TEXT,
   role user_role NOT NULL DEFAULT 'user',
   created_at TIMESTAMP NOT NULL DEFAULT now(),
   updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE oauth_tokens (
  id SERIAL PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider TEXT NOT NULL,
  refresh_token BYTEA NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE (user_id, provider)
);
