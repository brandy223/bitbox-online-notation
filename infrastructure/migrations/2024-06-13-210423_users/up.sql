-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO $$ BEGIN IF NOT EXISTS (
  SELECT
    1
  FROM
    pg_type
  WHERE
    typname = 'user_role'
) THEN CREATE TYPE user_role AS ENUM ('user', 'admin');
END IF;
END $$;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    username VARCHAR (255) NOT NULL,
    email VARCHAR (255) NOT NULL,
    has_validated_email BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role user_role NOT NULL DEFAULT 'user',
    token_version INT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);
CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);

CREATE TABLE IF NOT EXISTS user_passwords (
    user_id UUID PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    password VARCHAR (255) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
