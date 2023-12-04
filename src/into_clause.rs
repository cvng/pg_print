use crate::fmt;
use pg_query::protobuf::IntoClause;

impl fmt::Print for IntoClause {
    fn print(&self, p: &mut fmt::Printer) {
        if let Some(rel) = &self.rel {
            rel.print(p);
        }

        if !self.col_names.is_empty() {
            p.word(" (");
            p.column_list(&self.col_names);
            p.word(")");
        }

        if !self.access_method.is_empty() {
            p.word("using ");
            p.ident(self.access_method.clone());
            p.word(" ");
        }

        p.opt_with(&self.options);

        self.on_commit().print(p);

        if !self.table_space_name.is_empty() {
            p.word("tablespace ");
            p.ident(self.table_space_name.clone());
            p.word(" ");
        }
    }
}
