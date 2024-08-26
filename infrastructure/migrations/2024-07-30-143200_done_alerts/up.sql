-- Your SQL goes here
DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM pg_type
                       WHERE typname = 'alert_type') THEN CREATE TYPE alert_type AS ENUM
            ('started', 'pending', 'finished');
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS done_alerts (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    description VARCHAR(32),
    project_id UUID NOT NULL REFERENCES projects(id),
    type alert_type NOT NULL,
    published_at TIMESTAMP NOT NULL DEFAULT NOW()
);