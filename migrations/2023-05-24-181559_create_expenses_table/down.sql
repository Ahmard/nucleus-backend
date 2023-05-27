ALTER TABLE expenses DROP CONSTRAINT fk_expense_user_id;
ALTER TABLE expenses DROP CONSTRAINT fk_expense_project_id;
ALTER TABLE expenses DROP CONSTRAINT fk_expense_budget_id;
DROP TABLE expenses;