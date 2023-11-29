use crate::fmt;
use pg_query::protobuf::RangeVar;

impl fmt::Print for RangeVar {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        p.ident(self.relname.clone());
        Some(())
    }
}