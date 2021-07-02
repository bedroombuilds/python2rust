import psycopg2
import psycopg2.extras
import helper

if __name__ == '__main__':
    con = psycopg2.connect("host=localhost user=htd")
    cursor = con.cursor(cursor_factory=psycopg2.extras.DictCursor)
    ins_cursor = con.cursor()
    cursor.execute(helper.MOST_RECOVERED)
    for row in cursor:
        print('most recoveries', dict(row))

    ins_cursor.execute("DELETE FROM month_cases")
    cursor.execute(helper.GROUP_BY_MONTH)
    for row in cursor:
        print(tuple(row))
        ins_cursor.execute(
            '''INSERT INTO month_cases
            (mon, new_cases, recovered)
            VALUES (%s, %s, %s)''', row)

    con.commit()
