use crate::fmt::Printer;
use pg_query::protobuf::RangeVar;

impl Printer {
    pub fn range_var(&mut self, n: &RangeVar) {
        self.ident(n.relname.clone());
    }
}
