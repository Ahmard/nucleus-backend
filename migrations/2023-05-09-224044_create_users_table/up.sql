CREATE TABLE users
(
    user_id    UUID         NOT NULL UNIQUE PRIMARY KEY,
    first_name VARCHAR(150) NOT NULL,
    last_name  VARCHAR(150) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(250) NOT NULL,
    status     VARCHAR(50)  NOT NULL DEFAULT 'active',
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP             DEFAULT NULL
);
