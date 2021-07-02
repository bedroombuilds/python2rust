use std::collections::HashMap;
use tokio_postgres::{types::Type, Error, NoTls};

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
async fn row_to_str_vec(row: &tokio_postgres::Row) -> Vec<String> {
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

async fn row_to_str_hashmap(row: &tokio_postgres::Row) -> HashMap<String, String> {
    let values = row_to_str_vec(row).await;
    values
        .into_iter()
        .zip(row.columns().iter())
        .map(|(v, k)| (k.name().to_string(), v))
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (mut client, connection) =
        tokio_postgres::connect("host=localhost user=htd", NoTls).await?;

    // connection object performs communication with DB, spawned to run on its own
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query(MOST_RECOVERED, &[]).await?;

    let value: chrono::NaiveDate = rows[0].get(0);
    println!("{:?}", value);
    println!("{:?}", row_to_str_vec(&rows[0]).await);
    println!("{:?}", row_to_str_hashmap(&rows[0]).await);

    let transaction = client.transaction().await?;
    transaction.query("DELETE FROM month_cases", &[]).await?;
    let ins_mv = transaction
        .prepare(
            "INSERT INTO month_cases
            (mon, new_cases, recovered)
            VALUES ($1, $2, $3)",
        )
        .await?;

    for row in transaction.query(GROUP_BY_MONTH, &[]).await? {
        println!("{:?}", row_to_str_vec(&row).await);
        let date: chrono::NaiveDate = row.get(0);
        let new_cases: i64 = row.get(1);
        let recovered: i64 = row.get(2);
        transaction
            .execute(
                &ins_mv,
                &[&date.to_string(), &(new_cases as i32), &(recovered as i32)],
            )
            .await?;
    }
    transaction.commit().await?;
    Ok(())
}
