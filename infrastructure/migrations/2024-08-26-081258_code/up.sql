-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS mfa_codes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    code VARCHAR (16) NOT NULL,
    iat TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    exp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '10 minutes',
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE
);