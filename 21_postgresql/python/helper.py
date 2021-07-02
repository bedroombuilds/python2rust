MOST_RECOVERED = """SELECT * FROM cases
WHERE daily_recovered = (SELECT max(daily_recovered) FROM cases);"""
GROUP_BY_MONTH = """
SELECT date_trunc('month', d)::date as month,
    sum(daily_confirmed) as new_cases,
    sum(daily_recovered) as recovered
FROM cases
GROUP BY month
ORDER BY month"""
