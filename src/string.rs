use crate::fmt;
use pg_query::protobuf;

impl fmt::Print for protobuf::String {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.word("'");
        p.ident(self.sval.clone());
        p.word("'");
        Ok(())
    }
}
