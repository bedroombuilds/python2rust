use rusqlite::{params, Connection, Result};
mod helpers;

static MOST_RECOVERED: &str = r#"SELECT * FROM cases
WHERE daily_recovered = (SELECT max(daily_recovered) FROM cases);"#;
static GROUP_BY_MONTH: &str = r#"
SELECT strftime('%Y-%m', d) as month,
 sum(daily_confirmed) as new_cases,
 sum(daily_recovered) as recovered
  FROM cases GROUP BY month;
"#;

fn main() -> Result<()> {
    let conn = Connection::open("../../cts.sqlite3")?;
    let mut stmt = conn.prepare(MOST_RECOVERED)?;
    let row = stmt.query_row([], helpers::row_to_str_vec)?;
    println!("{:?}", row);
    let row = stmt.query_row([], helpers::row_to_hashmap)?;
    println!("most recoveries {:?}", row);

    conn.execute("DELETE FROM month_cases", [])?;
    let mut stmt = conn.prepare(GROUP_BY_MONTH)?;
    let month_cases = stmt.query_map([],
        |row| Ok(helpers::MonthCase::from_row(row)))?;
    for mc in month_cases {
        let mc = mc?;
        println!("{}", mc);
        conn.execute(
            "INSERT INTO month_cases
            (mon, new_cases, recovered)
            VALUES (?1, ?2, ?3)",
            params![mc.month, mc.new_cases, mc.recovered],
        )?;
    }

    Ok(())
}
