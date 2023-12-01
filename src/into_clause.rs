use crate::fmt;
use crate::fmt::gram;
use pg_query::protobuf::IntoClause;

impl fmt::Print for IntoClause {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if let Some(rel) = &self.rel {
            rel.print(p)?;
        }

        if !self.col_names.is_empty() {
            p.word(" (");
            gram::column_list(p, &self.col_names)?;
            p.word(")");
        }

        if !self.access_method.is_empty() {
            p.word("using ");
            p.ident(self.access_method.clone());
            p.word(" ");
        }

        gram::opt_with(p, &self.options)?;

        self.on_commit().print(p)?;

        if !self.table_space_name.is_empty() {
            p.word("tablespace ");
            p.ident(self.table_space_name.clone());
            p.word(" ");
        }

        Ok(())
    }
}
