mod deparse;
mod error;

pub fn unparse(sql: &str) -> String {
    deparse::deparse(sql).unwrap_or(sql.to_string())
}
