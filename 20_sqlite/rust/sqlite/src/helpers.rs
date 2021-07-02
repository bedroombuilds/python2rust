use std::collections::HashMap;

/// helper to make any row into a vector of strings
pub fn row_to_str_vec(row: &rusqlite::Row)
        -> Result<Vec<String>, rusqlite::Error> {
    let types = row.columns();
    Ok((0..row.column_count())
        .into_iter()
        .map(|ci| match types.get(ci).unwrap().decl_type() {
            Some("timestamp") => row.get_unwrap::<usize, String>(ci),
            Some("integer") => 
                format!("{}", row.get_unwrap::<usize, u32>(ci)),
            _ => String::from("t-unknown"),
        })
        .collect::<Vec<_>>())
}

/// helper to make any row into a HashMap where k, v are Strings
pub fn row_to_hashmap(row: &rusqlite::Row)
        -> Result<HashMap<String, String>, rusqlite::Error> {
    let types = row.columns();
    Ok((0..row.column_count())
        .into_iter()
        .map(|ci| {
            let col_name = row.column_name(ci).unwrap().to_owned();
            match types.get(ci).unwrap().decl_type() {
                Some("timestamp") =>
                    (col_name, row.get_unwrap::<usize, String>(ci)),
                Some("integer") =>
            (col_name, format!("{}", row.get_unwrap::<usize, u32>(ci))),
                _ => (col_name, String::from("t-unknown")),
            }
        })
        .collect::<HashMap<_, _>>())
}

pub struct MonthCase {
    pub month: String,
    pub new_cases: u32,
    pub recovered: u32,
}

impl MonthCase {
    /// constructor from a rusqlite Row
    pub fn from_row(row: &rusqlite::Row) -> Self {
        MonthCase {
            month: row.get(0).unwrap(),
            new_cases: row.get(1).unwrap(),
            recovered: row.get(2).unwrap(),
        }
    }
}

impl std::fmt::Display for MonthCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.month, self.new_cases, self.recovered)
    }
}
