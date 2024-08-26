-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS promotions
(
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title      VARCHAR(255) NOT NULL,
    start_year DATE         NOT NULL,
    end_year   DATE         NOT NULL,
    teacher_id UUID         NOT NULL REFERENCES users (id) ON DELETE CASCADE
);