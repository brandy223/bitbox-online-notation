-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM pg_type
                       WHERE typname = 'project_state') THEN CREATE TYPE project_state AS ENUM
            ('not-started', 'in-progress', 'finished', 'notation-finished');
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS projects
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL,
    description TEXT,
    start_date TIMESTAMP NOT NULL DEFAULT NOW(),
    end_date TIMESTAMP NOT NULL,
    notation_period_duration INT NOT NULL DEFAULT 7,
    promotion_id UUID NOT NULL REFERENCES promotions(id) ON DELETE CASCADE,
    state project_state NOT NULL DEFAULT 'not-started'
);