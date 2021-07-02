import sqlite3

MOST_RECOVERED = '''SELECT * FROM cases
WHERE daily_recovered = (SELECT max(daily_recovered) FROM cases);'''
GROUP_BY_MONTH = '''
SELECT strftime('%Y-%m', d) as month,
 sum(daily_confirmed) as new_cases,
 sum(daily_recovered) as recovered
  FROM cases GROUP BY month;
'''

if __name__ == '__main__':
    con = sqlite3.connect('../cts.sqlite3')
    con.row_factory = sqlite3.Row
    cursor = con.cursor()
    ins_cursor = con.cursor()
    for row in cursor.execute(MOST_RECOVERED):
        print('most recoveries',
              {k: v for k, v in zip(row.keys(), tuple(row))})

    ins_cursor.execute("DELETE FROM month_cases")
    for row in cursor.execute(GROUP_BY_MONTH):
        print(tuple(row))
        ins_cursor.execute(
            '''INSERT INTO month_cases
            (mon, new_cases, recovered)
            VALUES (?, ?, ?)''', row)

    con.commit()
