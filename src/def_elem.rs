use crate::fmt::Printer;
use pg_query::protobuf::DefElem;

impl Printer {
    pub fn def_elem(&mut self, n: &DefElem) {
        if !n.defnamespace.is_empty() {
            self.ident(n.defnamespace.clone());
            self.word(".");
        }

        self.ident(n.defname.clone());

        if let Some(arg) = &n.arg {
            self.word(" = ");
            self.node(arg);
        }
    }
}
