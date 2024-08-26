-- Your SQL goes here
CREATE TABLE IF NOT EXISTS marks
(
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    noted_student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    grader_student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    mark FLOAT NOT NULL,
    max_mark INTEGER NOT NULL DEFAULT 20,
    comment TEXT,
    PRIMARY KEY (project_id, group_id, noted_student_id, grader_student_id)
);