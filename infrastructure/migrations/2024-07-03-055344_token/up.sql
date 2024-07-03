-- Your SQL goes here
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM pg_type
                       WHERE typname = 'token_type') THEN CREATE TYPE token_type AS ENUM
            ('pass-reset', 'student-marks', 'account-activation', 'email-verification');
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS tokens
(
    id   TEXT PRIMARY KEY NOT NULL,
    type token_type       NOT NULL,
    used BOOLEAN          NOT NULL DEFAULT FALSE
);