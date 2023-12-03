use crate::fmt;
use pg_query::protobuf::AccessPriv;

impl fmt::Print for AccessPriv {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if !self.priv_name.is_empty() {
            match self.priv_name.as_ref() {
                "select" => p.word("select"),
                "references" => p.word("references"),
                "create" => p.word("create"),
                _ => p.ident(self.priv_name.clone()),
            }
        } else {
            p.word("all")
        }

        p.nbsp();

        if !self.cols.is_empty() {
            p.word("(");
            p.column_list(&self.cols)?;
            p.word(")");
        }

        Ok(())
    }
}
