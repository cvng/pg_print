use crate::fmt;
use pg_query::protobuf;

impl fmt::Print for protobuf::String {
    fn print(&self, p: &mut fmt::Printer) {
        p.word("'");
        p.ident(self.sval.clone());
        p.word("'");
    }
}
