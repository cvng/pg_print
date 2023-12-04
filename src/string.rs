use crate::fmt::Printer;
use pg_query::protobuf;

impl Printer {
    pub fn string(&mut self, n: &protobuf::String) {
        self.word("'");
        self.ident(n.sval.clone());
        self.word("'");
    }
}
