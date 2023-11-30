use crate::fmt;
use pg_query::protobuf::RawStmt;

impl fmt::Print for RawStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        self.stmt.as_ref().unwrap().print(p)
    }
}
