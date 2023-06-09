CREATE TABLE expenses
(
    expense_id UUID          NOT NULL UNIQUE PRIMARY KEY,
    user_id    UUID          NOT NULL,
    project_id UUID          NOT NULL,
    budget_id  UUID          NOT NULL,
    amount     BIGINT        NOT NULL,
    narration  VARCHAR(1000) NOT NULL,
    spent_at   TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP              DEFAULT NULL
);

ALTER TABLE expenses
    ADD CONSTRAINT fk_expense_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);

ALTER TABLE expenses
    ADD CONSTRAINT fk_expense_project_id FOREIGN KEY (project_id) REFERENCES projects (project_id);

ALTER TABLE expenses
    ADD CONSTRAINT fk_expense_budget_id FOREIGN KEY (budget_id) REFERENCES budgets (budget_id);
