use crate::fmt;
use pg_query::protobuf::DefElem;

impl fmt::Print for DefElem {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if !self.defnamespace.is_empty() {
            p.ident(self.defnamespace.clone());
            p.word(".");
        }

        p.ident(self.defname.clone());

        if let Some(arg) = &self.arg {
            p.word(" = ");
            p.node(arg);
        }

        Ok(())
    }
}
