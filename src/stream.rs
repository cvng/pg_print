#[derive(Default)]
pub struct IndentedStream {}

impl IndentedStream {
    pub fn new() -> Self {
        Self {}
    }

    pub fn call(&self, sql: &str) -> String {
        sql.to_string()
    }
}
