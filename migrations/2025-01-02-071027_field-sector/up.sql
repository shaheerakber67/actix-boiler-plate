-- Create the field_sector table
CREATE TABLE field_sector (
    id SERIAL PRIMARY KEY,
    created_by INT NULL,
    last_modified_by INT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified_date TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'INACTIVE'))
);

-- Create the field_sector_translation table
CREATE TABLE field_sector_translation (
    id SERIAL PRIMARY KEY,
    field_sector_id INT NOT NULL REFERENCES field_sector (id) ON DELETE CASCADE,
    created_by INT NULL,
    last_modified_by INT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified_date TIMESTAMP NOT NULL DEFAULT NOW(),
    lang_code VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);
