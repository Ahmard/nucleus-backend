CREATE TABLE project_labels
(
    project_label_id CHAR(36)  NOT NULL UNIQUE PRIMARY KEY,
    user_id          CHAR(36)  NOT NULL,
    project_id       CHAR(36)  NOT NULL,
    label_id         CHAR(36)  NOT NULL,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at       TIMESTAMP          DEFAULT NULL
);

ALTER TABLE project_labels
    ADD CONSTRAINT fk_project_label_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);

ALTER TABLE project_labels
    ADD CONSTRAINT fk_project_label_project_id FOREIGN KEY (project_id) REFERENCES projects (project_id);

ALTER TABLE project_labels
    ADD CONSTRAINT fk_project_label_label_id FOREIGN KEY (label_id) REFERENCES labels (label_id);
