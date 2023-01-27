CREATE TABLE IF NOT EXISTS users(
    id INT PRIMARY KEY,
    first_name TEXT NOT NULL,
    address TEXT,
    phone_number TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    role TEXT NOT NULL,
    is_blocked BOOL,
    is_deleted BOOL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS courier(
    id INTEGER NOT NULL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id INT NOT NULL,
    is_free BOOL NOT NULL,
    rating INT NOT NULL
);


CREATE OR REPLACE FUNCTION check_user_update()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := current_timestamp;
    RETURN NEW;
END;
    $$ LANGUAGE PLPGSQL;

CREATE TRIGGER check_update
    AFTER UPDATE ON users
    FOR EACH ROW
EXECUTE FUNCTION check_user_update();