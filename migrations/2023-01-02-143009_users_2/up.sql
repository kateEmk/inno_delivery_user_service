ALTER TABLE users ALTER COLUMN is_blocked SET NOT NULL;
ALTER TABLE users ALTER COLUMN is_deleted SET NOT NULL;
ALTER TABLE users ALTER COLUMN updated_at SET NOT NULL;

ALTER TABLE courier ALTER COLUMN rating TYPE FLOAT;
ALTER TABLE courier ALTER COLUMN rating SET NOT NULL;
