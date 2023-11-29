use crate::create_stmt::node_column_list;
use crate::create_stmt::node_opt_with;
use crate::fmt;
use pg_query::protobuf::IntoClause;

impl fmt::Print for IntoClause {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        self.rel.as_ref()?.print(p)?;

        if !self.col_names.is_empty() {
            p.word(" (");
            node_column_list(p, &self.col_names);
            p.word(")");
        }

        if !self.access_method.is_empty() {
            p.word("using ");
            p.ident(self.access_method.clone());
            p.word(" ");
        }

        node_opt_with(p, &self.options);

        self.on_commit().print(p)?;

        if !self.table_space_name.is_empty() {
            p.word("tablespace ");
            p.ident(self.table_space_name.clone());
            p.word(" ");
        }

        Some(())
    }
}
