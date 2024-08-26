-- Your SQL goes here
-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS students_tokens
(
    id    UUID PRIMARY KEY    DEFAULT uuid_generate_v4(),
    token TEXT       NOT NULL,
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    used  BOOLEAN    NOT NULL DEFAULT FALSE
);