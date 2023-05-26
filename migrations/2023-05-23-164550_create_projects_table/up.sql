CREATE TABLE projects
(
    project_id  UUID     NOT NULL UNIQUE PRIMARY KEY,
    user_id     UUID     NOT NULL,
    name        VARCHAR(150) NOT NULL,
    description VARCHAR(150) NOT NULL,
    created_at  TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at  TIMESTAMP             DEFAULT NULL
);

ALTER TABLE projects
    ADD CONSTRAINT fk_project_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
