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
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS courier(
    id INTEGER NOT NULL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id INT NOT NULL,
    rating INT NOT NULL
);

CREATE FUNCTION check_user_update()
    RETURNS trigger AS
    LANGUAGE PLPGSQL
    AS
    $$
    BEGIN
         NEW.first_name <> OLD.first_name THEN
		 INSERT INTO users(user_id,first_name,updated_at)
		 VALUES(OLD.id,OLD.first_name,now());
	END IF;
	    RETURN NEW;

    END;
    $$


CREATE TRIGGER check_update
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION check_user_update();
