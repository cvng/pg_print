use crate::fmt;
use pg_query::protobuf::ParseResult;

impl fmt::Print for ParseResult {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        p.cbox(0);
        for stmt in &self.stmts {
            stmt.print(p)?;
        }
        p.end();

        Some(())
    }
}
