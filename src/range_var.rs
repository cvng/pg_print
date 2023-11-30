use crate::fmt;
use pg_query::protobuf::RangeVar;

impl fmt::Print for RangeVar {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        Ok(p.ident(self.relname.clone()))
    }
}
