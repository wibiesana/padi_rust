CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,
    username VARCHAR(50) UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    remember_token VARCHAR(100),
    last_login_at VARCHAR(19),
    -- Any driver can't decode DATETIME/TIMESTAMP: store as 'YYYY-MM-DD HH:MM:SS' text.
    created_at VARCHAR(19),
    updated_at VARCHAR(19)
);
-- +down
DROP TABLE users;
