CREATE TABLE labels
(
    label_id   UUID         NOT NULL UNIQUE PRIMARY KEY,
    user_id    UUID     NOT NULL,
    name       VARCHAR(150) NOT NULL,
    module     VARCHAR(50)  NOT NULL DEFAULT 'projects',
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP             DEFAULT NULL
);

ALTER TABLE labels
    ADD CONSTRAINT fk_label_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
