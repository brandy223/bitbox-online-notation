-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM pg_type
                       WHERE typname = 'token_type') THEN CREATE TYPE token_type AS ENUM
            ('pass-reset', 'account-activation', 'email-verification');
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS tokens
(
    id    UUID PRIMARY KEY    DEFAULT uuid_generate_v4(),
    token TEXT       NOT NULL,
    type  token_type NOT NULL,
    used  BOOLEAN    NOT NULL DEFAULT FALSE
);