CREATE TYPE channel_type as ENUM('text', 'voice');

CREATE TABLE users (
  id BIGINT PRIMARY KEY UNIQUE, -- snowflakes
  username VARCHAR(40) NOT NULL UNIQUE, -- slightly higher length, for funsies
  email VARCHAR(255) NOT NULL UNIQUE, -- quoted theoretically there's no limit on email length, but..
  servers BIGINT[] NOT NULL
);

CREATE TABLE servers (
  id BIGINT PRIMARY KEY UNIQUE, -- snowflakes
  name VARCHAR(72) NOT NULL,
  owner_uid BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  users BIGINT[] NOT NULL
);

CREATE TABLE channels (
  id BIGINT PRIMARY KEY UNIQUE, -- snowflakes
  server_id BIGINT NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
  name VARCHAR(72) NOT NULL,
  type channel_type NOT NULL DEFAULT 'text'
);

CREATE TABLE messages (
  id BIGINT PRIMARY KEY UNIQUE, -- snowflakes
  author_id BIGINT NOT NULL REFERENCES users (id) ON DELETE SET NULL,
  channel_id BIGINT NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
  content JSONB NOT NULL -- message struct as json for searchability
);

CREATE TABLE user_authentication (
  user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE UNIQUE PRIMARY KEY,
  password_hash VARCHAR(60) NOT NULL, -- sha512 encrypt + bcrypt
  mfa_enabled BOOLEAN NOT NULL DEFAULT false,
  mfa_b32 BYTEA,
  mfa_auth_url TEXT
);

CREATE TABLE user_tokens (
  token TEXT NOT NULL PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  scopes TEXT[] NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP NOT NULL
);
