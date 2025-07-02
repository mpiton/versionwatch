-- Add migration script here
CREATE TABLE software_versions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    current_version VARCHAR(255),
    latest_version VARCHAR(255) NOT NULL,
    latest_lts_version VARCHAR(255),
    is_lts BOOLEAN DEFAULT FALSE,
    eol_date DATE,
    release_notes_url TEXT,
    cve_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON software_versions
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();
