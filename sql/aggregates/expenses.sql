SELECT (SELECT SUM(amount) FROM expenses WHERE EXTRACT(YEAR FROM expenses.spent_at) = ?)            AS year_expenses,
       (SELECT SUM(amount) FROM expenses WHERE EXTRACT(MONTH FROM expenses.spent_at) = ?)           AS month_expenses,
       (SELECT SUM(amount) FROM expenses WHERE EXTRACT(DAY FROM expenses.spent_at) = ?)             AS today_expenses,
       (SELECT SUM(amount) FROM expenses WHERE EXTRACT(WEEK FROM NOW()) = EXTRACT(WEEK FROM NOW())) AS week_expenses;

SELECT (SELECT SUM(amount) FROM expenses WHERE EXTRACT(YEAR FROM expenses.spent_at) = 2023)::VARCHAR AS year_expenses;