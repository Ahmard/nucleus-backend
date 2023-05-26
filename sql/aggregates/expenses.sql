SELECT (SELECT SUM(amount)
        FROM expenses
        WHERE EXTRACT(YEAR FROM expenses.spent_at) = ? AND expenses.user_id = ?)                                   AS year_expenses,
       (SELECT SUM(amount)
        FROM expenses
        WHERE EXTRACT(MONTH FROM expenses.spent_at) = ?
          AND expenses.user_id = ?)                                                                                AS month_expenses,
       (SELECT SUM(amount)
        FROM expenses
        WHERE EXTRACT(DAY FROM expenses.spent_at) = ?
          AND expenses.user_id = ?)                                                                                AS today_expenses,
       (SELECT SUM(amount)
        FROM expenses
        WHERE EXTRACT(WEEK FROM NOW()) = EXTRACT(WEEK FROM NOW())
          AND expenses.user_id = ?)                                                                                AS week_expenses;

SELECT (SELECT SUM(amount) FROM expenses WHERE EXTRACT(YEAR FROM expenses.spent_at) = 2023)::VARCHAR AS year_expenses;