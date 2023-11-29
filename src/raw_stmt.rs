use crate::fmt;
use pg_query::protobuf::RawStmt;

impl fmt::Print for RawStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        self.stmt.as_ref()?.node.as_ref()?.print(p)
    }
}
