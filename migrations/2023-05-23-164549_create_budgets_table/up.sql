CREATE TABLE budgets
(
    budget_id   UUID         NOT NULL UNIQUE PRIMARY KEY,
    user_id     UUID         NOT NULL,
    amount      BIGINT       NOT NULL,
    amount_used BIGINT       NOT NULL,
    month       SMALLINT     NOT NULL,
    year        SMALLINT     NOT NULL,
    title       VARCHAR(150) NOT NULL DEFAULT NULL,
    comment     VARCHAR(250) NULL     DEFAULT NULL,
    created_at  TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at  TIMESTAMP    NULL     DEFAULT NULL
);

ALTER TABLE budgets
    ADD CONSTRAINT fk_budget_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
