-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS groups
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL,
    mark FLOAT,
    max_mark INTEGER NOT NULL DEFAULT 20,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS groups_students
(
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    student_mark FLOAT,
    max_mark INTEGER NOT NULL DEFAULT 20,
    PRIMARY KEY (group_id, student_id)
);