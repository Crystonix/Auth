CREATE TYPE user_role AS ENUM ('user', 'admin');

CREATE TABLE users (
   id TEXT PRIMARY KEY,
   username TEXT NOT NULL,
   discriminator TEXT NOT NULL,
   avatar TEXT,
   role user_role NOT NULL DEFAULT 'user'
);

CREATE TABLE oauth_tokens (
    user_id TEXT PRIMARY KEY REFERENCES users(id),
    refresh_token TEXT
);
