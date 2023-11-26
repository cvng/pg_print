use crate::error::Result;

pub(crate) fn deparse(sql: &str) -> Result<String> {
    Ok(sql.to_string())
}
