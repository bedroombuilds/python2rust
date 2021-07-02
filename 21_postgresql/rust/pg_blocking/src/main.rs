use postgres::{types::Type, Client, Error, NoTls};
use std::collections::HashMap;

static MOST_RECOVERED: &str = r#"SELECT * FROM cases
WHERE daily_recovered = (SELECT max(daily_recovered) FROM cases);"#;
static GROUP_BY_MONTH: &str = "
SELECT date_trunc('month', d)::date as month,
    sum(daily_confirmed) as new_cases,
    sum(daily_recovered) as recovered
FROM cases
GROUP BY month
ORDER BY month";

/// silly way to convert row into Vec of Strings from postgres types
/// see https://docs.rs/postgres/latest/postgres/types/trait.FromSql.html
fn row_to_str_vec(row: &postgres::Row) -> Vec<String> {
    (0..row.len())
        .into_iter()
        .map(|ci| match *row.columns().get(ci).unwrap().type_() {
            Type::DATE => row.get::<usize, chrono::NaiveDate>(ci).to_string(),
            Type::INT4 => row.get::<usize, i32>(ci).to_string(),
            Type::INT8 => row.get::<usize, i64>(ci).to_string(),
            _ => "blah".to_string(),
        })
        .collect()
}

fn row_to_str_hashmap(row: &postgres::Row) -> HashMap<String, String> {
    let values = row_to_str_vec(row);
    values
        .into_iter()
        .zip(row.columns().iter())
        .map(|(v, k)| (k.name().to_string(), v))
        .collect()
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=htd", NoTls)?;

    let rows = client.query(MOST_RECOVERED, &[])?;

    let value: chrono::NaiveDate = rows[0].get(0);
    println!("{:?}", value);
    println!("{:?}", row_to_str_vec(&rows[0]));
    println!("{:?}", row_to_str_hashmap(&rows[0]));

    let mut transaction = client.transaction()?;
    transaction.query("DELETE FROM month_cases", &[])?;
    let ins_mv = transaction.prepare(
        "INSERT INTO month_cases
            (mon, new_cases, recovered)
            VALUES ($1, $2, $3)",
    )?;
    for row in transaction.query(GROUP_BY_MONTH, &[])? {
        println!("{:?}", row_to_str_vec(&row));
        let date: chrono::NaiveDate = row.get(0);
        let new_cases: i64 = row.get(1);
        let recovered: i64 = row.get(2);
        transaction.execute(
            &ins_mv,
            &[&date.to_string(), &(new_cases as i32), &(recovered as i32)],
        )?;
    }
    transaction.commit()?;
    Ok(())
}
