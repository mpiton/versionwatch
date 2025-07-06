-- Drop the old table and related trigger/function if they exist
DROP TRIGGER IF EXISTS set_timestamp ON software_versions;
DROP TABLE IF EXISTS software_versions;
DROP FUNCTION IF EXISTS trigger_set_timestamp;

-- Create products table
-- This table stores information about the tracked products.
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL, -- e.g., "postgresql", "rust"
    display_name VARCHAR(255), -- e.g., "PostgreSQL", "Rust"
    homepage VARCHAR(255),
    documentation_url VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create cycles table
-- This table stores the release cycle details for each product.
CREATE TABLE cycles (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL, -- e.g., "16", "15" for PostgreSQL
    release_date DATE,
    eol_date DATE,
    lts BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(product_id, name)
);

-- Create a function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to automatically update the 'updated_at' column for both tables
CREATE TRIGGER set_timestamp_products
BEFORE UPDATE ON products
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TRIGGER set_timestamp_cycles
BEFORE UPDATE ON cycles
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp(); 