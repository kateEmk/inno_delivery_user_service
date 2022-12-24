CREATE TABLE IF NOT EXISTS users(
    id INT PRIMARY KEY,
    first_name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    role TEXT NOT NULL,
    is_blocked BOOL,
    refresh_token VARCHAR
);

CREATE TABLE IF NOT EXISTS addresses(
    id INT PRIMARY KEY,
    user_id INT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    address TEXT NOT NULL
)