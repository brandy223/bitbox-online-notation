-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS main_config
(
    id                 SERIAL PRIMARY KEY,
    register           BOOLEAN   NOT NULL DEFAULT TRUE,
    authorized_domains TEXT[]    NOT NULL DEFAULT ARRAY[]::TEXT[],
    updated_at         TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS user_config
(
    id         SERIAL PRIMARY KEY,
    user_id    UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    alerts     JSON[]    NOT NULL DEFAULT ARRAY[]::JSON[],
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION update_timestamp()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_main_config_timestamp
    BEFORE UPDATE ON main_config
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TRIGGER update_user_config_timestamp
    BEFORE UPDATE ON user_config
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();