-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL UNIQUE,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS blogs (
    id bigserial PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    content text NOT NULL DEFAULT '',
    author_id bigint NOT NULL REFERENCES users(id),
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_created_at ON blogs (author_id, created_at);

CREATE INDEX idx_user_updated_at ON blogs (author_id, updated_at);

CREATE INDEX idx_user_title ON blogs (author_id, title);