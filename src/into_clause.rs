use crate::fmt::Printer;
use pg_query::protobuf::IntoClause;

impl Printer {
    #[allow(clippy::wrong_self_convention)]
    pub fn into_clause(&mut self, n: &IntoClause) {
        if let Some(rel) = &n.rel {
            self.range_var(rel);
        }

        if !n.col_names.is_empty() {
            self.word(" (");
            self.column_list(&n.col_names);
            self.word(")");
        }

        if !n.access_method.is_empty() {
            self.word("using ");
            self.ident(n.access_method.clone());
            self.word(" ");
        }

        self.opt_with(&n.options);

        self.on_commit_action(&n.on_commit());

        if !n.table_space_name.is_empty() {
            self.word("tablespace ");
            self.ident(n.table_space_name.clone());
            self.word(" ");
        }
    }
}
